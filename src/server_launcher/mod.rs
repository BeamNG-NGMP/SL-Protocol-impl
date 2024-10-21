pub mod generic;
pub mod handshake;
pub mod serverinfo;

use generic::*;
use handshake::*;
use serverinfo::*;

use crate::*;

pub struct PacketHeader {
    pub sig_a: char,
    pub sig_b: char,
    pub packet_length: u32,
}

pub enum Packet {
    Confirmation(ConfirmationPacket),

    Version(VersionPacket),
    Authentication(AuthenticationPacket),

    ServerInfo(ServerInfoPacket),
    ModList(ModListPacket),
    LoadMap(LoadMapPacket),
}

impl PacketTrait for Packet {
    fn from_raw(sig_a: char, sig_b: char, packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        match (sig_a, sig_b) {
            ('C', 'C') => Ok(Self::Confirmation(ConfirmationPacket::from_raw(packet_data)?)),

            ('V', 'C') => Ok(Self::Version(VersionPacket::from_raw(packet_data)?)),
            ('A', 'C') => Ok(Self::Authentication(AuthenticationPacket::from_raw(packet_data)?)),

            ('H', 'I') => Ok(Self::ServerInfo(ServerInfoPacket::from_raw(packet_data)?)),
            ('M', 'L') => Ok(Self::ModList(ModListPacket::from_raw(packet_data)?)),
            ('L', 'M') => Ok(Self::LoadMap(LoadMapPacket::from_raw(packet_data)?)),

            _ => Err(PacketDecodeError::UnknownPacket(sig_a, sig_b)),
        }
    }

    fn to_raw(self) -> Result<(char, char, Vec<u8>), PacketEncodeError> {
        match self {
            Self::Confirmation(p) => Ok(('C', 'C', p.to_raw()?)),

            Self::Version(p) => Ok(('V', 'C', p.to_raw()?)),
            Self::Authentication(p) => Ok(('A', 'C', p.to_raw()?)),

            Self::ServerInfo(p) => Ok(('H', 'I', p.to_raw()?)),
            Self::ModList(p) => Ok(('M', 'L', p.to_raw()?)),
            Self::LoadMap(p) => Ok(('L', 'M', p.to_raw()?)),
        }
    }
}
