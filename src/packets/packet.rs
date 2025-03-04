use std::sync::{Arc, Mutex};
use crate::database::Database;
use crate::device::Device;
use crate::player::Player;
use crate::reader::*;
use crate::settings::*;

pub trait ClientPacket {
    fn decode(&mut self, stream: &mut ByteReader) -> Result<(), DecodeError>;
    fn process(&mut self, device: &mut Device, player: &mut Player, database: &mut Arc<Mutex<Database>>, settings: &Settings);
}

pub trait ServerPacket {
    fn encode(&mut self) -> Vec<u8>;
}
