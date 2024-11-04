use super::{PacketDecodeError, PacketEncodeError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    pub name: String,
    pub steam_id: String,
    pub avatar_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDataPacket {
    pub players: Vec<PlayerData>,
}

impl PlayerDataPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json)
            .map_err(|e| PacketDecodeError::InvalidJson("VehicleSpawnPacket", e))
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json =
            serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleData {
    #[serde(rename = "Jbeam")]
    pub jbeam: String,
    pub object_id: u32,
    pub paints: String,
    #[serde(rename = "partConfig")]
    pub part_config: String,
    pub pos: [f32; 3],
    pub rot: [f32; 4],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleSpawnPacket {
    pub confirm_id: u16,
    pub steam_id: String,
    pub vehicle_id: u16,
    pub vehicle_data: VehicleData,
}

impl VehicleSpawnPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json)
            .map_err(|e| PacketDecodeError::InvalidJson("VehicleSpawnPacket", e))
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json =
            serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleConfirmPacket {
    pub confirm_id: u16,
    pub vehicle_id: u16,
    pub object_id: u32,
}

impl VehicleConfirmPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json)
            .map_err(|e| PacketDecodeError::InvalidJson("VehicleConfirmPacket", e))
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json =
            serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleDeletePacket {
    pub steam_id: String,
    pub vehicle_id: u16,
}

impl VehicleDeletePacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json)
            .map_err(|e| PacketDecodeError::InvalidJson("VehicleDeletePacket", e))
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json =
            serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleTransformPacket {
    pub steam_id: String,
    pub vehicle_id: u16,
    pub transform: String,
}

impl VehicleTransformPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json)
            .map_err(|e| PacketDecodeError::InvalidJson("VehicleTransformPacket", e))
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json =
            serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleUpdatePacket {
    pub steam_id: String,
    pub vehicle_id: u16,
    pub runtime_data: String,
}

impl VehicleUpdatePacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json)
            .map_err(|e| PacketDecodeError::InvalidJson("VehicleUpdatePacket", e))
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json =
            serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}
