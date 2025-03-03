use crate::packets::{
    packet::*,
    client::*,
};

pub fn create_packet(packet_id: u16) -> Option<Box<dyn ClientPacket>> {
    match packet_id {
        10101 => Some(Box::new(LoginMessage::default())),
        _ => None,
    }
}
