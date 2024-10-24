use super::{PacketDecodeError, PacketEncodeError};

use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct VersionPacket {
    pub confirm_id: u16,
    pub protocol_version: u16
}

impl VersionPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let data_len = packet_data.len();
        if data_len != 4 { return Err(PacketDecodeError::InvalidDataSize { expected: 4, actual: data_len }); }
        let confirm_id = u16::from_le_bytes([packet_data[0], packet_data[1]]);
        let protocol_version = u16::from_le_bytes([packet_data[2], packet_data[3]]);
        Ok(Self {
            confirm_id,
            protocol_version,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut buf = Vec::with_capacity(4);
        buf.append(&mut self.confirm_id.to_le_bytes().to_vec());
        buf.append(&mut self.protocol_version.to_le_bytes().to_vec());
        Ok(buf)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfoPacket {
    pub confirm_id: u16,
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
    pub confirm_id: u16,
    pub success: bool,
    pub player_name: String,
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
