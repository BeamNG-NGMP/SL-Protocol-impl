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
    pub steam_id: u64,
    pub vehicle_id: u16,
    pub vehicle_data: VehicleData,
}

impl VehicleSpawnPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json).map_err(|_| PacketDecodeError::InvalidJson)
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json = serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}

#[derive(Debug)]
pub struct VehicleConfirmPacket {
    pub confirm_id: u16,
    pub vehicle_id: u16,
    pub obj_id: u32,
}

impl VehicleConfirmPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let mut pd = packet_data.into_iter();
        let confirm_id = u16::from_le_bytes([pd.next().ok_or(PacketDecodeError::UnexpectedEof)?, pd.next().ok_or(PacketDecodeError::UnexpectedEof)?]);
        let vehicle_id = u16::from_le_bytes([pd.next().ok_or(PacketDecodeError::UnexpectedEof)?, pd.next().ok_or(PacketDecodeError::UnexpectedEof)?]);
        let obj_id = u32::from_le_bytes([
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
            ]);
        Ok(Self {
            confirm_id,
            vehicle_id,
            obj_id,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut bytes = self.confirm_id.to_le_bytes().to_vec();
        bytes.extend_from_slice(&self.vehicle_id.to_le_bytes());
        bytes.extend_from_slice(&self.obj_id.to_le_bytes());
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
        let mut pd = packet_data.into_iter();
        let player_id = u64::from_le_bytes([
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,

                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
            ]);
        let vehicle_id = u16::from_le_bytes([pd.next().ok_or(PacketDecodeError::UnexpectedEof)?, pd.next().ok_or(PacketDecodeError::UnexpectedEof)?]);
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

#[derive(Debug)]
pub struct VehicleTransformPacket {
    pub player_id: u64,
    pub vehicle_id: u16,
    pub transform: String,
}

impl VehicleTransformPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let mut pd = packet_data.into_iter();
        let player_id = u64::from_le_bytes([
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,

                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
            ]);
        let vehicle_id = u16::from_le_bytes([pd.next().ok_or(PacketDecodeError::UnexpectedEof)?, pd.next().ok_or(PacketDecodeError::UnexpectedEof)?]);
        let transform = String::from_utf8(pd.collect::<Vec<u8>>()).map_err(|_| PacketDecodeError::InvalidString)?;
        Ok(Self {
            player_id,
            vehicle_id,
            transform,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut bytes = self.player_id.to_le_bytes().to_vec();
        bytes.extend_from_slice(&self.vehicle_id.to_le_bytes());
        bytes.extend_from_slice(&self.transform.as_bytes());
        Ok(bytes)
    }
}

#[derive(Debug)]
pub struct VehicleUpdatePacket {
    pub player_id: u64,
    pub vehicle_id: u16,
    pub runtime_data: String,
}

impl VehicleUpdatePacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let mut pd = packet_data.into_iter();
        let player_id = u64::from_le_bytes([
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,

                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
                pd.next().ok_or(PacketDecodeError::UnexpectedEof)?,
            ]);
        let vehicle_id = u16::from_le_bytes([pd.next().ok_or(PacketDecodeError::UnexpectedEof)?, pd.next().ok_or(PacketDecodeError::UnexpectedEof)?]);
        let runtime_data = String::from_utf8(pd.collect::<Vec<u8>>()).map_err(|_| PacketDecodeError::InvalidString)?;
        Ok(Self {
            player_id,
            vehicle_id,
            runtime_data,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut bytes = self.player_id.to_le_bytes().to_vec();
        bytes.extend_from_slice(&self.vehicle_id.to_le_bytes());
        bytes.extend_from_slice(&self.runtime_data.as_bytes());
        Ok(bytes)
    }
}
