use super::{PacketDecodeError, PacketEncodeError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionPacket {
    pub protocol_version: u16
}

impl VersionPacket {
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
pub struct ClientInfoPacket {
    pub userfolder: String,
    pub client_version: u16,
}

impl ClientInfoPacket {
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
pub struct AuthenticationInfoPacket {
    pub success: bool,
    pub player_name: String,
    pub steam_id: String,
    pub avatar_hash: String,
}

impl AuthenticationInfoPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let json = String::from_utf8(packet_data).map_err(|_| PacketDecodeError::InvalidString)?;
        serde_json::from_str(&json).map_err(|_| PacketDecodeError::InvalidJson)
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let json = serde_json::to_string(&self).map_err(|_| PacketEncodeError::CannotSerializeJson)?;
        Ok(json.as_bytes().to_vec())
    }
}
