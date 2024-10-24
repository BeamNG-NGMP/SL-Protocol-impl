use super::{PacketDecodeError, PacketEncodeError};

#[derive(Debug)]
pub struct ConfirmationPacket {
    pub confirm_id: u16,
}

impl ConfirmationPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let data_len = packet_data.len();
        if data_len != 2 { return Err(PacketDecodeError::InvalidDataSize { expected: 2, actual: data_len }); }
        let confirm_id = u16::from_le_bytes([packet_data[0], packet_data[1]]);
        Ok(Self {
            confirm_id,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        Ok(self.confirm_id.to_le_bytes().to_vec())
    }
}

#[derive(Debug)]
pub struct JoinServerPacket {
    pub confirm_id: u16,
    pub ip_addr: String,
}

impl JoinServerPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let data_len = packet_data.len();
        if data_len < 2 { return Err(PacketDecodeError::InvalidDataSize { expected: 2, actual: data_len }); }
        let confirm_id = u16::from_le_bytes([packet_data[0], packet_data[1]]);
        let ip_addr = String::from_utf8(packet_data[2..].to_vec()).map_err(|_| PacketDecodeError::InvalidString)?;
        Ok(Self {
            confirm_id,
            ip_addr,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut bytes = self.confirm_id.to_le_bytes().to_vec();
        bytes.extend_from_slice(self.ip_addr.as_bytes());
        Ok(bytes)
    }
}

#[derive(Debug)]
pub struct LoadMapPacket {
    pub confirm_id: u16,
    pub map_name: String,
}

impl LoadMapPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let data_len = packet_data.len();
        if data_len < 2 { return Err(PacketDecodeError::InvalidDataSize { expected: 2, actual: data_len }); }

        let confirm_id = u16::from_le_bytes([packet_data[0], packet_data[1]]);

        // TODO: String::with_capacity for 1 single big allocation for extra performance?
        let map_name = String::from_utf8(packet_data[2..].to_vec()).map_err(|_| PacketDecodeError::InvalidString)?;
        Ok(Self {
            confirm_id,
            map_name,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut buf = self.confirm_id.to_le_bytes().to_vec();
        for c in self.map_name.chars().into_iter() {
            buf.push(c as u8);
        }
        Ok(buf)
    }
}
