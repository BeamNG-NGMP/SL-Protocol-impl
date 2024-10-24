use super::{PacketDecodeError, PacketEncodeError};

#[derive(Debug)]
pub struct ServerInfoPacket {
    pub http_port: u16,
    pub udp_port: u16,
}

impl ServerInfoPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let data_len = packet_data.len();
        if data_len != 4 { return Err(PacketDecodeError::InvalidDataSize { expected: 4, actual: data_len }); }
        let http_port = u16::from_le_bytes([packet_data[0], packet_data[1]]);
        let udp_port = u16::from_le_bytes([packet_data[2], packet_data[3]]);
        Ok(Self {
            http_port,
            udp_port,
        })
    }

    pub fn to_raw(self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut bytes = self.http_port.to_le_bytes().to_vec();
        bytes.extend_from_slice(&self.udp_port.to_le_bytes());
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

    pub fn to_raw(self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut buf = self.confirm_id.to_le_bytes().to_vec();
        for c in self.map_name.chars().into_iter() {
            buf.push(c as u8);
        }
        Ok(buf)
    }
}
