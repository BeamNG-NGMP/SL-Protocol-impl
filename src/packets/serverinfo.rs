use super::{PacketDecodeError, PacketEncodeError};

pub struct HttpInfoPacket {
    pub http_port: u16
}

impl HttpInfoPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let data_len = packet_data.len();
        if data_len != 2 { return Err(PacketDecodeError::InvalidDataSize { expected: 2, actual: data_len }); }
        let http_port = u16::from_le_bytes([packet_data[0], packet_data[1]]);
        Ok(Self {
            http_port,
        })
    }

    pub fn to_raw(self) -> Result<Vec<u8>, PacketEncodeError> {
        Ok(self.http_port.to_le_bytes().to_vec())
    }
}
