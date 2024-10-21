#[macro_use] extern crate log;

pub mod server_launcher;
pub mod connection;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketDecodeError {
    #[error("unknown packet ({0}{1})")]
    UnknownPacket(char, char),
    #[error("unexpected data size (expected {expected:?}, got {actual:?}")]
    InvalidDataSize {
        expected: usize,
        actual: usize
    },
}

#[derive(Error, Debug)]
pub enum PacketEncodeError {
    #[error("generic")]
    Generic,
}

pub trait PacketTrait: Sized {
    fn from_raw(sig_a: char, sig_b: char, packet_data: Vec<u8>) -> Result<Self, PacketDecodeError>;
    fn to_raw(self) -> Result<(char, char, Vec<u8>), PacketEncodeError>;
}
