use super::{PacketDecodeError, PacketEncodeError};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    pub name: String,
    pub steam_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDataPacket {
    pub players: Vec<PlayerData>,
}

impl PlayerDataPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let raw = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;

        Ok(serde_json::from_str(&raw).map_err(|_| PacketDecodeError::InvalidJson)?)
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        Ok(serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?.as_bytes().to_vec())
    }
}

#[derive(Debug)]
pub struct VehicleSpawnPacket {
    pub confirm_id: u16,

    pub raw_json: String,
}

impl VehicleSpawnPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let data_len = packet_data.len();
        if data_len < 2 { return Err(PacketDecodeError::InvalidDataSize { expected: 2, actual: data_len }); }
        let confirm_id = u16::from_le_bytes([packet_data[0], packet_data[1]]);
        let raw_json = String::from_utf8(packet_data[2..].to_vec()).map_err(|_| PacketDecodeError::InvalidString)?;
        Ok(Self {
            confirm_id,
            raw_json,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut bytes = self.confirm_id.to_le_bytes().to_vec();
        bytes.extend_from_slice(self.raw_json.as_bytes());
        Ok(bytes)
    }
}

#[derive(Debug)]
pub struct VehicleConfirmPacket {
    pub confirm_id: u16,
    pub vehicle_id: u16,
}

impl VehicleConfirmPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let data_len = packet_data.len();
        if data_len != 4 { return Err(PacketDecodeError::InvalidDataSize { expected: 4, actual: data_len }); }
        let confirm_id = u16::from_le_bytes([packet_data[0], packet_data[1]]);
        let vehicle_id = u16::from_le_bytes([packet_data[2], packet_data[3]]);
        Ok(Self {
            confirm_id,
            vehicle_id,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut bytes = self.confirm_id.to_le_bytes().to_vec();
        bytes.extend_from_slice(&self.vehicle_id.to_le_bytes());
        Ok(bytes)
    }
}

#[derive(Debug)]
pub struct VehicleDeletePacket {
    pub player_id: u64,
    pub vehicle_id: u16,
}

impl VehicleDeletePacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let data_len = packet_data.len();
        if data_len != 10 { return Err(PacketDecodeError::InvalidDataSize { expected: 10, actual: data_len }); }
        let player_id = u64::from_le_bytes([
                packet_data[0],
                packet_data[1],
                packet_data[2],
                packet_data[3],

                packet_data[4],
                packet_data[5],
                packet_data[6],
                packet_data[7],
            ]);
        let vehicle_id = u16::from_le_bytes([packet_data[8], packet_data[9]]);
        Ok(Self {
            player_id,
            vehicle_id,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut bytes = self.player_id.to_le_bytes().to_vec();
        bytes.extend_from_slice(&self.vehicle_id.to_le_bytes());
        Ok(bytes)
    }
}
