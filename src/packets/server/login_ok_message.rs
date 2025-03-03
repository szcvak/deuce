use crate::packets::client::LoginMessage;
use crate::packets::packet::ServerPacket;
use crate::writer::ByteWriter;

pub struct LoginOkMessage<'a> {
    pub id: u16,
    payload: &'a LoginMessage,
}

impl<'a> LoginOkMessage<'a> {
    pub fn new(payload: &'a LoginMessage) -> Self {
        Self {
            id: 20104,
            payload,
        }
    }
}

impl<'a> ServerPacket for LoginOkMessage<'a> {
    fn encode(&mut self) -> Vec<u8> {
        let mut writer = ByteWriter::new();
        
        writer.write_long(self.payload.high_id as i32, self.payload.low_id as i32);
        writer.write_long(self.payload.high_id as i32, self.payload.low_id as i32);

        writer.write_string(Some(self.payload.token.as_str()));
        writer.write_string(Some("467606826913688"));
        writer.write_string(Some("G:325378671"));
        
        writer.write_int(self.payload.major_version as i32);
        writer.write_int(self.payload.minor_version as i32);
        writer.write_int(self.payload.build as i32);
        
        writer.write_string(Some("-dev"));
        
        writer.write_int(0);
        writer.write_int(0);
        writer.write_int(0);
        
        writer.write_string(None);
        writer.write_string(None);
        writer.write_string(None);

        writer.write_int(0);

        writer.write_string(None);
        writer.write_string(Some(self.payload.region.as_str()));
        writer.write_string(None);

        writer.write_int(1);
        
        writer.write_string(None);
        writer.write_string(None);
        writer.write_string(None);

        writer.buffer
    }
}
