use thiserror::Error;

pub mod generic;
pub mod handshake;
pub mod serverinfo;

use generic::*;
use handshake::*;
use serverinfo::*;

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

pub enum Packet {
    Confirmation(ConfirmationPacket),

    Version(VersionPacket),
    Authentication(AuthenticationPacket),

    HttpInfo(HttpInfoPacket),
    ModList(ModListPacket),
    LoadMap(LoadMapPacket),
}

impl Packet {
    pub fn from_raw(sig_a: char, sig_b: char, packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        match (sig_a, sig_b) {
            ('C', 'C') => Ok(Self::Confirmation(ConfirmationPacket::from_raw(packet_data)?)),

            ('V', 'C') => Ok(Self::Version(VersionPacket::from_raw(packet_data)?)),
            ('A', 'C') => Ok(Self::Authentication(AuthenticationPacket::from_raw(packet_data)?)),

            ('H', 'I') => Ok(Self::HttpInfo(HttpInfoPacket::from_raw(packet_data)?)),
            ('M', 'L') => Ok(Self::ModList(ModListPacket::from_raw(packet_data)?)),
            ('L', 'M') => Ok(Self::LoadMap(LoadMapPacket::from_raw(packet_data)?)),

            _ => Err(PacketDecodeError::UnknownPacket(sig_a, sig_b)),
        }
    }

    pub fn to_raw(self) -> Result<(char, char, Vec<u8>), PacketEncodeError> {
        match self {
            Self::Confirmation(p) => Ok(('C', 'C', p.to_raw()?)),

            Self::Version(p) => Ok(('V', 'C', p.to_raw()?)),
            Self::Authentication(p) => Ok(('A', 'C', p.to_raw()?)),

            Self::HttpInfo(p) => Ok(('H', 'I', p.to_raw()?)),
            Self::ModList(p) => Ok(('M', 'L', p.to_raw()?)),
            Self::LoadMap(p) => Ok(('L', 'M', p.to_raw()?)),
        }
    }
}
