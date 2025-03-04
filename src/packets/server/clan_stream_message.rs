use crate::packets::packet::ServerPacket;
use crate::writer::ByteWriter;

pub struct ClanStreamMessage {
    pub id: u16,
}

impl ClanStreamMessage {
    pub fn new() -> Self {
        Self {
            id: 24311,
        }
    }
}

impl ServerPacket for ClanStreamMessage {
    fn encode(&mut self) -> Vec<u8> {
        let mut writer = ByteWriter::new();
        
        writer.write_vint(0);
        writer.write_vint(0);

        writer.buffer
    }
}
