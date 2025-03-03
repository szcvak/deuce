use crate::packets::client::LoginMessage;
use crate::packets::packet::ServerPacket;
use crate::writer::ByteWriter;

pub struct LoginFailedMessage<'a> {
    pub id: u16,
    
    pub message: String,
    pub error_code: i32,
    
    payload: &'a LoginMessage,
}

impl<'a> LoginFailedMessage<'a> {
    pub fn new(payload: &'a LoginMessage, message: String, error_code: i32) -> Self {
        Self {
            id: 20103,
            
            message,
            error_code,
            
            payload,
        }
    }
}

impl<'a> ServerPacket for LoginFailedMessage<'a> {
    fn encode(&mut self) -> Vec<u8> {
        let mut writer = ByteWriter::new();
        
        writer.write_int(self.error_code);
        writer.write_string(Some(self.payload.fingerprint_sha.as_str()));
        
        writer.write_string(Some("0.0.0.0:9339")); // TODO: change dynamically
        writer.write_string(Some("https://game-assets.brawlstarsgame.com"));
        writer.write_string(Some("https://github.com/Super-brawl-team/Obiad-Brawl"));
        
        writer.write_int(0);
        writer.write_boolean(false);
        
        writer.write_string(None);
        writer.write_string(None);
        
        writer.write_int(0);
        writer.write_int(3);

        writer.write_string(None);
        writer.write_string(None);
        
        writer.write_int(0);
        writer.write_int(0);

        writer.write_boolean(false);
        writer.write_boolean(false);
        
        writer.buffer
    }
}
