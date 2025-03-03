use crate::packets::packet::{ClientPacket, ServerPacket};
use crate::reader::{ByteReader, DecodeError};
use log::*;
use crate::device::Device;
use crate::packets::server::LoginFailedMessage;

#[derive(Default, Debug)]
pub struct LoginMessage {
    pub high_id: u32,
    pub low_id: u32,
    pub token: String,
    pub major_version: u32,
    pub minor_version: u32,
    pub build: u32,
    pub fingerprint_sha: String,
    pub unknown_string1: String,
    pub device_id: String,
    pub unknown_string2: String,
    pub device: String,
    pub system_language: u32,
    pub region: String,
}

impl ClientPacket for LoginMessage {
    fn decode(&mut self, stream: &mut ByteReader) -> Result<(), DecodeError> {
        self.high_id = stream.read_u32()?;
        self.low_id = stream.read_u32()?;

        self.token = stream.read_string()?;

        self.major_version = stream.read_u32()?;
        self.minor_version = stream.read_u32()?;
        self.build = stream.read_u32()?;

        self.fingerprint_sha = stream.read_string()?;
        self.unknown_string1 = stream.read_string()?;

        self.device_id = stream.read_string()?;
        self.unknown_string2 = stream.read_string()?;

        self.device = stream.read_string()?;
        self.system_language = stream.read_vint()? as u32;

        let lang = stream.read_string()?;

        let mut parts = lang.split('-');
        parts.next();

        self.region = parts.next().unwrap_or("").to_string();

        Ok(())
    }

    fn process(&mut self, device: &mut Device) {
        info!("Requested processing for LoginMessage. Sending LoginFailedMessage.");
        
        let mut msg = LoginFailedMessage::new(self, " ".to_string(), 8);
        let encoded = msg.encode();
        
        device.send(msg.id, encoded, 0);
    }
}
