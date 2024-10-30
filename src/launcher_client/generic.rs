use super::{PacketDecodeError, PacketEncodeError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmationPacket {
    pub confirm_id: u16,
}

impl ConfirmationPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json).map_err(|_| PacketDecodeError::InvalidJson)
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json = serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinServerPacket {
    pub ip_address: String,
}

impl JoinServerPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json).map_err(|_| PacketDecodeError::InvalidJson)
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json = serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionErrorPacket {
    pub error: String,
}

impl ConnectionErrorPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json).map_err(|_| PacketDecodeError::InvalidJson)
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json = serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadMapPacket {
    pub confirm_id: u16,
    pub map_string: String,
}

impl LoadMapPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json).map_err(|_| PacketDecodeError::InvalidJson)
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json = serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}
