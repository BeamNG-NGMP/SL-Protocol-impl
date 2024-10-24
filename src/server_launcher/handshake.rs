use super::{PacketDecodeError, PacketEncodeError};

#[derive(Debug)]
pub struct VersionPacket {
    pub confirm_id: u16,
    pub client_version: u16
}

impl VersionPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let data_len = packet_data.len();
        if data_len != 4 { return Err(PacketDecodeError::InvalidDataSize { expected: 4, actual: data_len }); }
        let confirm_id = u16::from_le_bytes([packet_data[0], packet_data[1]]);
        let client_version = u16::from_le_bytes([packet_data[2], packet_data[3]]);
        Ok(Self {
            confirm_id,
            client_version,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut buf = Vec::with_capacity(4);
        buf.append(&mut self.confirm_id.to_le_bytes().to_vec());
        buf.append(&mut self.client_version.to_le_bytes().to_vec());
        Ok(buf)
    }
}

#[derive(Debug)]
pub struct AuthenticationPacket {
    pub confirm_id: u16,
    pub auth_code: String,
}

impl AuthenticationPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let data_len = packet_data.len();
        if data_len < 3 { return Err(PacketDecodeError::InvalidDataSize { expected: 3, actual: data_len }); }
        let confirm_id = u16::from_le_bytes([packet_data[0], packet_data[1]]);
        // TODO: String::with_capacity for 1 single big allocation for extra performance?
        let mut auth_code = String::new();
        for i in 2..packet_data.len() {
            auth_code.push(packet_data[i] as char);
        }
        Ok(Self {
            confirm_id,
            auth_code,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut buf = Vec::with_capacity(2 + self.auth_code.len());
        buf.append(&mut self.confirm_id.to_le_bytes().to_vec());
        for c in self.auth_code.chars().into_iter() {
            buf.push(c as u8);
        }
        Ok(buf)
    }
}
