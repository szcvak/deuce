use std::collections::HashMap;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use bytes::Bytes;
use log::*;
use crate::database::Database;
use crate::device::Device;
use crate::packets::client::create_packet;
use crate::player::Player;
use crate::reader::ByteReader;
use crate::settings::Settings;

pub struct ClientInfo {
    stream: TcpStream,
}

pub struct Network {
    settings: Arc<Settings>,
    clients: Arc<Mutex<HashMap<String, ClientInfo>>>,

    clients_count: Arc<AtomicUsize>,
    database: Arc<Mutex<Database>>,
}

impl Network {
    pub fn new(settings: Settings) -> Self {
        let db = Database::new(settings.database.as_str());
        
        Self {
            settings: Arc::new(settings),

            clients: Arc::new(Mutex::new(HashMap::new())),
            clients_count: Arc::new(AtomicUsize::new(0)),
            
            database: Arc::new(Mutex::new(db)),
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.settings.port))
            .expect("deuce: could not bind to 0.0.0.0");

        {
            let mut database = self.database.lock().unwrap();
            database.create_default();
        }

        for stream in listener.incoming() {
            if stream.is_err() {
                error!("deuce: incoming stream is erroneous, will skip");
                continue;
            }

            let mut stream = stream.unwrap();

            let count = self.clients_count.fetch_add(1, Ordering::SeqCst) + 1;
            let client_id = count.to_string();

            info!("deuce: client connected. total: {}", count);

            {
                let mut clients = self.clients.lock().unwrap();
                clients.insert(client_id.clone(), ClientInfo { stream: stream.try_clone().expect("deuce: failed to clone stream") });
            }

            let mut device = Device::new(self.settings.clone(), stream.try_clone().unwrap(), Arc::clone(&self.clients));
            let mut player = Player::new();

            let clients = Arc::clone(&self.clients);
            let clients_count = Arc::clone(&self.clients_count);
            let mut database = Arc::clone(&self.database);
            let settings = Arc::clone(&self.settings);

            std::thread::spawn(move || {
                loop {
                    let mut header = [0u8; 7];

                    if let Err(e) = stream.read_exact(&mut header) {
                        error!("deuce: failed to read stream header: {}", e);
                        break;
                    }

                    let packet_id = u16::from_be_bytes([header[0], header[1]]);
                    let length = ((header[2] as u32) << 16) | ((header[3] as u32) << 8) | (header[4] as u32);
                    let version = u16::from_be_bytes([header[5], header[6]]);

                    let mut payload = vec![0u8; length as usize];

                    if let Err(e) = stream.read_exact(&mut payload) {
                        error!("deuce: failed to read payload: {}", e);
                        break;
                    }

                    info!("deuce: received packet {} (bytes: {}, version: {})", packet_id, length, version);

                    device.decrypt(&mut payload);

                    let mut reader = ByteReader::from(Bytes::from(payload));

                    if let Some(mut packet) = create_packet(packet_id) {
                        if let Err(e) = packet.decode(&mut reader) {
                            error!("deuce: failed to decode packet {}: {:?}", packet_id, e);
                        } else {
                            packet.process(&mut device, &mut player, &mut database, &settings);
                        }
                    }
                }

                clients_count.fetch_sub(1, Ordering::SeqCst);

                let mut map = clients.lock().unwrap();
                map.remove(&client_id);

                info!("deuce: client disconnected. total: {}", clients_count.load(Ordering::SeqCst));
            });
        }
    }
}
