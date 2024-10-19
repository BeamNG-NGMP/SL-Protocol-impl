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

pub struct ModListPacket {
    pub mods: Vec<String>,
}

impl ModListPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let mut pd_iter = packet_data.into_iter().map(|raw| raw as char);
        let mut mods = Vec::new();
        let mut mod_name = String::new();
        while let Some(c) = pd_iter.next() {
            if c == ';' {
                mods.push(mod_name);
                mod_name = String::new();
            }
            mod_name.push(c);
        }
        if mod_name.is_empty() == false {
            mods.push(mod_name);
        }
        Ok(Self {
            mods,
        })
    }

    pub fn to_raw(self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut buf = Vec::new();
        for c in self.mods.join(";").chars().into_iter() {
            buf.push(c as u8);
        }
        Ok(buf)
    }
}

pub struct LoadMapPacket {
    pub map_name: String,
}

impl LoadMapPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        // TODO: String::with_capacity for 1 single big allocation for extra performance?
        let mut map_name = String::new();
        for i in 0..packet_data.len() {
            map_name.push(packet_data[i] as char);
        }
        Ok(Self {
            map_name,
        })
    }

    pub fn to_raw(self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut buf = Vec::new();
        for c in self.map_name.chars().into_iter() {
            buf.push(c as u8);
        }
        Ok(buf)
    }
}
