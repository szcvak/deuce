use crate::reader::*;

pub trait ClientPacket {
    fn decode(&mut self, stream: &mut ByteReader) -> Result<(), DecodeError>;
    fn process(&mut self);
}

pub trait ServerPacket {
    fn encode(&mut self) -> Vec<u8>;
}
