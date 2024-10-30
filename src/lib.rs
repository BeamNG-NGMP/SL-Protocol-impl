#[macro_use] extern crate log;

pub mod connection;
pub mod server_launcher;
pub mod launcher_client;

use thiserror::Error;

pub struct PacketHeader {
    pub sig_a: char,
    pub sig_b: char,
    pub packet_length: u32,
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("invalid packet size")]
    InvalidPacketSize,
}

#[derive(Error, Debug)]
pub enum PacketDecodeError {
    #[error("unknown packet ({0}{1})")]
    UnknownPacket(char, char),
    #[error("unexpected data size (expected {expected:?}, got {actual:?})")]
    InvalidDataSize {
        expected: usize,
        actual: usize
    },
    #[error("unexpected eof")]
    UnexpectedEof,
    #[error("invalid string")]
    InvalidString,
    #[error("invalid json")]
    InvalidJson,
    #[error("invalid number")]
    InvalidNumber,
}

#[derive(Error, Debug)]
pub enum PacketEncodeError {
    #[error("cannot serialize to json")]
    CannotSerializeJson,
}

pub trait PacketTrait: Sized {
    fn from_raw(sig_a: char, sig_b: char, packet_data: Vec<u8>) -> Result<Self, PacketDecodeError>;
    fn to_raw(&self) -> Result<(char, char, Vec<u8>), PacketEncodeError>;
}
