use crate::*;

pub mod gameplay;
pub mod generic;
pub mod handshake;

use gameplay::*;
use generic::*;
use handshake::*;

#[derive(Debug)]
pub enum Packet {
    ReloadLauncherConnection,

    Confirmation(ConfirmationPacket),
    ConnectionError(ConnectionErrorPacket),

    Version(VersionPacket),
    ClientInfo(ClientInfoPacket),
    AuthenticationInfo(AuthenticationInfoPacket),
    LoginRequest,

    JoinServer(JoinServerPacket),
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
            ('R', 'L') => Ok(Self::ReloadLauncherConnection),

            ('C', 'C') => Ok(Self::Confirmation(ConfirmationPacket::from_raw(
                packet_data,
            )?)),
            ('C', 'E') => Ok(Self::ConnectionError(ConnectionErrorPacket::from_raw(
                packet_data,
            )?)),

            ('V', 'C') => Ok(Self::Version(VersionPacket::from_raw(packet_data)?)),

            ('C', 'I') => Ok(Self::ClientInfo(ClientInfoPacket::from_raw(packet_data)?)),
            ('A', 'I') => Ok(Self::AuthenticationInfo(
                AuthenticationInfoPacket::from_raw(packet_data)?,
            )),
            ('L', 'R') => Ok(Self::LoginRequest),

            ('H', 'J') => Ok(Self::JoinServer(JoinServerPacket::from_raw(packet_data)?)),
            ('L', 'M') => Ok(Self::LoadMap(LoadMapPacket::from_raw(packet_data)?)),

            ('P', 'D') => Ok(Self::PlayerData(PlayerDataPacket::from_raw(packet_data)?)),

            ('V', 'S') => Ok(Self::VehicleSpawn(VehicleSpawnPacket::from_raw(
                packet_data,
            )?)),
            ('V', 'A') => Ok(Self::VehicleConfirm(VehicleConfirmPacket::from_raw(
                packet_data,
            )?)),
            ('V', 'D') => Ok(Self::VehicleDelete(VehicleDeletePacket::from_raw(
                packet_data,
            )?)),

            ('V', 'T') => Ok(Self::VehicleTransform(VehicleTransformPacket::from_raw(
                packet_data,
            )?)),
            ('V', 'U') => Ok(Self::VehicleUpdate(VehicleUpdatePacket::from_raw(
                packet_data,
            )?)),

            _ => Err(PacketDecodeError::UnknownPacket(sig_a, sig_b)),
        }
    }

    fn to_raw(&self) -> Result<(char, char, Vec<u8>), PacketEncodeError> {
        match self {
            Self::ReloadLauncherConnection => Ok(('R', 'L', Vec::new())),

            Self::Confirmation(p) => Ok(('C', 'C', p.to_raw()?)),
            Self::ConnectionError(p) => Ok(('C', 'E', p.to_raw()?)),

            Self::Version(p) => Ok(('V', 'C', p.to_raw()?)),

            Self::ClientInfo(p) => Ok(('C', 'I', p.to_raw()?)),
            Self::AuthenticationInfo(p) => Ok(('A', 'I', p.to_raw()?)),
            Self::LoginRequest => Ok(('L', 'R', Vec::new())),

            Self::JoinServer(p) => Ok(('H', 'J', p.to_raw()?)),
            Self::LoadMap(p) => Ok(('L', 'M', p.to_raw()?)),

            Self::PlayerData(p) => Ok(('P', 'D', p.to_raw()?)),

            Self::VehicleSpawn(p) => Ok(('V', 'S', p.to_raw()?)),
            Self::VehicleConfirm(p) => Ok(('V', 'A', p.to_raw()?)),
            Self::VehicleDelete(p) => Ok(('V', 'D', p.to_raw()?)),

            Self::VehicleTransform(p) => Ok(('V', 'T', p.to_raw()?)),
            Self::VehicleUpdate(p) => Ok(('V', 'U', p.to_raw()?)),
        }
    }
}
