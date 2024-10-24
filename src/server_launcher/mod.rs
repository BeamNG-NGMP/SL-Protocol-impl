use crate::*;

pub mod generic;
pub mod handshake;
pub mod serverinfo;
pub mod gameplay;

use generic::*;
use handshake::*;
use serverinfo::*;
use gameplay::*;

#[derive(Debug)]
pub enum Packet {
    Confirmation(ConfirmationPacket),

    Version(VersionPacket),
    Authentication(AuthenticationPacket),

    ServerInfo(ServerInfoPacket),
    LoadMap(LoadMapPacket),

    PlayerData(PlayerDataPacket),
}

impl PacketTrait for Packet {
    fn from_raw(sig_a: char, sig_b: char, packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        match (sig_a, sig_b) {
            ('C', 'C') => Ok(Self::Confirmation(ConfirmationPacket::from_raw(packet_data)?)),

            ('V', 'C') => Ok(Self::Version(VersionPacket::from_raw(packet_data)?)),
            ('A', 'C') => Ok(Self::Authentication(AuthenticationPacket::from_raw(packet_data)?)),

            ('H', 'I') => Ok(Self::ServerInfo(ServerInfoPacket::from_raw(packet_data)?)),
            ('L', 'M') => Ok(Self::LoadMap(LoadMapPacket::from_raw(packet_data)?)),

            ('P', 'L') => Ok(Self::PlayerData(PlayerDataPacket::from_raw(packet_data)?)),

            _ => Err(PacketDecodeError::UnknownPacket(sig_a, sig_b)),
        }
    }

    fn to_raw(&self) -> Result<(char, char, Vec<u8>), PacketEncodeError> {
        match self {
            Self::Confirmation(p) => Ok(('C', 'C', p.to_raw()?)),

            Self::Version(p) => Ok(('V', 'C', p.to_raw()?)),
            Self::Authentication(p) => Ok(('A', 'C', p.to_raw()?)),

            Self::ServerInfo(p) => Ok(('H', 'I', p.to_raw()?)),
            Self::LoadMap(p) => Ok(('L', 'M', p.to_raw()?)),

            Self::PlayerData(p) => Ok(('P', 'L', p.to_raw()?)),
        }
    }
}
