use crate::device::Device;
use crate::reader::*;

pub trait ClientPacket {
    fn decode(&mut self, stream: &mut ByteReader) -> Result<(), DecodeError>;
    fn process(&mut self, device: &mut Device);
}

pub trait ServerPacket {
    fn encode(&mut self) -> Vec<u8>;
}
