use crate::packets::packet::ServerPacket;
use crate::writer::ByteWriter;

pub struct MyAllianceMessage {
    pub id: u16,
}

impl MyAllianceMessage {
    pub fn new() -> Self {
        Self {
            id: 24311,
        }
    }
}

impl ServerPacket for MyAllianceMessage {
    fn encode(&mut self) -> Vec<u8> {
        let mut writer = ByteWriter::new();
        
        writer.write_vint(0);
        writer.write_boolean(false);

        writer.buffer
    }
}
