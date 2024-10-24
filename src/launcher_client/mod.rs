use crate::*;

pub mod generic;
pub mod handshake;
pub mod gameplay;

use generic::*;
use handshake::*;
use gameplay::*;

#[derive(Debug)]
pub enum Packet {
    ReloadLauncherConnection,

    Confirmation(ConfirmationPacket),
    Version(VersionPacket),
    ClientInfo(ClientInfoPacket),
    AuthenticationInfo(AuthenticationInfoPacket),

    JoinServer(JoinServerPacket),
    LoadMap(LoadMapPacket),

    VehicleSpawn(VehicleSpawnPacket),
}

impl PacketTrait for Packet {
    fn from_raw(sig_a: char, sig_b: char, packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        match (sig_a, sig_b) {
            ('R', 'L') => Ok(Self::ReloadLauncherConnection),

            ('C', 'C') => Ok(Self::Confirmation(ConfirmationPacket::from_raw(packet_data)?)),

            ('V', 'C') => Ok(Self::Version(VersionPacket::from_raw(packet_data)?)),

            ('C', 'I') => Ok(Self::ClientInfo(ClientInfoPacket::from_raw(packet_data)?)),
            ('A', 'I') => Ok(Self::AuthenticationInfo(AuthenticationInfoPacket::from_raw(packet_data)?)),

            ('H', 'J') => Ok(Self::JoinServer(JoinServerPacket::from_raw(packet_data)?)),
            ('L', 'M') => Ok(Self::LoadMap(LoadMapPacket::from_raw(packet_data)?)),

            ('V', 'S') => Ok(Self::VehicleSpawn(VehicleSpawnPacket::from_raw(packet_data)?)),

            _ => Err(PacketDecodeError::UnknownPacket(sig_a, sig_b)),
        }
    }

    fn to_raw(&self) -> Result<(char, char, Vec<u8>), PacketEncodeError> {
        match self {
            Self::ReloadLauncherConnection => Ok(('R', 'L', Vec::new())),

            Self::Confirmation(p) => Ok(('C', 'C', p.to_raw()?)),

            Self::Version(p) => Ok(('V', 'C', p.to_raw()?)),

            Self::ClientInfo(p) => Ok(('C', 'I', p.to_raw()?)),
            Self::AuthenticationInfo(p) => Ok(('A', 'I', p.to_raw()?)),

            Self::JoinServer(p) => Ok(('H', 'J', p.to_raw()?)),
            Self::LoadMap(p) => Ok(('L', 'M', p.to_raw()?)),

            Self::VehicleSpawn(p) => Ok(('V', 'S', p.to_raw()?)),
        }
    }
}
