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
    PlayerKick(PlayerKickPacket),

    Version(VersionPacket),
    Authentication(AuthenticationPacket),

    ServerInfo(ServerInfoPacket),
    LoadMap(LoadMapPacket),

    PlayerData(PlayerDataPacket),

    VehicleSpawn(VehicleSpawnPacket),
    VehicleConfirm(VehicleConfirmPacket),
    VehicleDelete(VehicleDeletePacket),

    VehicleTransform(VehicleTransformPacket),
    VehicleUpdate(VehicleUpdatePacket),
}

impl PacketTrait for Packet {
    fn from_raw(sig_a: char, sig_b: char, packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        match (sig_a, sig_b) {
            ('C', 'C') => Ok(Self::Confirmation(ConfirmationPacket::from_raw(packet_data)?)),
            ('P', 'K') => Ok(Self::PlayerKick(PlayerKickPacket::from_raw(packet_data)?)),

            ('V', 'C') => Ok(Self::Version(VersionPacket::from_raw(packet_data)?)),
            ('A', 'C') => Ok(Self::Authentication(AuthenticationPacket::from_raw(packet_data)?)),

            ('H', 'I') => Ok(Self::ServerInfo(ServerInfoPacket::from_raw(packet_data)?)),
            ('L', 'M') => Ok(Self::LoadMap(LoadMapPacket::from_raw(packet_data)?)),

            ('P', 'L') => Ok(Self::PlayerData(PlayerDataPacket::from_raw(packet_data)?)),

            ('V', 'S') => Ok(Self::VehicleSpawn(VehicleSpawnPacket::from_raw(packet_data)?)),
            ('V', 'A') => Ok(Self::VehicleConfirm(VehicleConfirmPacket::from_raw(packet_data)?)),
            ('V', 'D') => Ok(Self::VehicleDelete(VehicleDeletePacket::from_raw(packet_data)?)),

            ('V', 'T') => Ok(Self::VehicleTransform(VehicleTransformPacket::from_raw(packet_data)?)),
            ('V', 'U') => Ok(Self::VehicleUpdate(VehicleUpdatePacket::from_raw(packet_data)?)),

            _ => Err(PacketDecodeError::UnknownPacket(sig_a, sig_b)),
        }
    }

    fn to_raw(&self) -> Result<(char, char, Vec<u8>), PacketEncodeError> {
        match self {
            Self::Confirmation(p) => Ok(('C', 'C', p.to_raw()?)),
            Self::PlayerKick(p) => Ok(('P', 'K', p.to_raw()?)),

            Self::Version(p) => Ok(('V', 'C', p.to_raw()?)),
            Self::Authentication(p) => Ok(('A', 'C', p.to_raw()?)),

            Self::ServerInfo(p) => Ok(('H', 'I', p.to_raw()?)),
            Self::LoadMap(p) => Ok(('L', 'M', p.to_raw()?)),

            Self::PlayerData(p) => Ok(('P', 'L', p.to_raw()?)),

            Self::VehicleSpawn(p) => Ok(('V', 'S', p.to_raw()?)),
            Self::VehicleConfirm(p) => Ok(('V', 'A', p.to_raw()?)),
            Self::VehicleDelete(p) => Ok(('V', 'D', p.to_raw()?)),

            Self::VehicleTransform(p) => Ok(('V', 'T', p.to_raw()?)),
            Self::VehicleUpdate(p) => Ok(('V', 'U', p.to_raw()?)),
        }
    }
}
