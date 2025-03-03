use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::network::ClientInfo;
use crate::rc4::Rc4;
use crate::settings::Settings;

pub struct Device {
    stream: TcpStream,
    clients: Arc<Mutex<HashMap<String, ClientInfo>>>,

    decryptor: Rc4,
    encryptor: Rc4,
}

impl Device {
    pub fn new(settings: &Settings, stream: TcpStream, clients: Arc<Mutex<HashMap<String, ClientInfo>>>) -> Self {
        let binding = settings.key.clone();
        let key = binding.as_bytes();
        let nonce = b"nonce";

        let mut full_key = Vec::with_capacity(key.len() + nonce.len());

        full_key.extend_from_slice(&key);
        full_key.extend_from_slice(nonce);

        let mut decryptor = Rc4::new(&full_key);
        let mut encryptor = Rc4::new(&full_key);

        decryptor.process(&mut full_key);
        encryptor.process(&mut full_key);

        Self {
            stream,
            clients,

            decryptor,
            encryptor,
        }
    }

    pub fn decrypt(&mut self, data: &mut [u8]) {
        self.decryptor.process(data);
    }

    pub fn encrypt(&mut self, data: &mut [u8]) {
        self.encryptor.process(data);
    }
}