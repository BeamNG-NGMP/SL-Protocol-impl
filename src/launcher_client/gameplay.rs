use super::{PacketDecodeError, PacketEncodeError};

#[derive(Debug)]
pub struct VehicleSpawnPacket {
    pub confirm_id: u16,

    pub raw_json: String,
}

impl VehicleSpawnPacket {
    pub fn from_raw(packet_data: Vec<u8>) -> Result<Self, PacketDecodeError> {
        let mut pd = packet_data.into_iter();
        let confirm_id = u16::from_le_bytes([pd.next().ok_or(PacketDecodeError::UnexpectedEof)?, pd.next().ok_or(PacketDecodeError::UnexpectedEof)?]);
        let raw_json = String::from_utf8(pd.collect::<Vec<u8>>()).map_err(|_| PacketDecodeError::InvalidString)?;
        Ok(Self {
            confirm_id,
            raw_json,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let mut bytes = self.confirm_id.to_le_bytes().to_vec();
        bytes.extend_from_slice(self.raw_json.as_bytes());
        Ok(bytes)
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
        let player_id_len = pd.next().ok_or(PacketDecodeError::UnexpectedEof)?;
        let mut player_id_str = String::new();
        for _ in 0..player_id_len {
            player_id_str.push(pd.next().ok_or(PacketDecodeError::UnexpectedEof)? as char);
        }
        let vehicle_id = u16::from_le_bytes([pd.next().ok_or(PacketDecodeError::UnexpectedEof)?, pd.next().ok_or(PacketDecodeError::UnexpectedEof)?]);

        let player_id: u64 = player_id_str.parse().map_err(|_| PacketDecodeError::InvalidNumber)?;

        Ok(Self {
            player_id,
            vehicle_id,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let player_id_str = self.player_id.to_string();
        let mut bytes = vec![player_id_str.len() as u8];
        bytes.extend_from_slice(&player_id_str.as_bytes());
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
        let player_id_len = pd.next().ok_or(PacketDecodeError::UnexpectedEof)?;
        let mut player_id_str = String::new();
        for _ in 0..player_id_len {
            player_id_str.push(pd.next().ok_or(PacketDecodeError::UnexpectedEof)? as char);
        }
        let vehicle_id = u16::from_le_bytes([pd.next().ok_or(PacketDecodeError::UnexpectedEof)?, pd.next().ok_or(PacketDecodeError::UnexpectedEof)?]);
        let transform = String::from_utf8(pd.collect::<Vec<u8>>()).map_err(|_| PacketDecodeError::InvalidString)?;

        let player_id: u64 = player_id_str.parse().map_err(|_| PacketDecodeError::InvalidNumber)?;

        Ok(Self {
            player_id,
            vehicle_id,
            transform,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let player_id_str = self.player_id.to_string();
        let mut bytes = vec![player_id_str.len() as u8];
        bytes.extend_from_slice(&player_id_str.as_bytes());
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
        let player_id_len = pd.next().ok_or(PacketDecodeError::UnexpectedEof)?;
        let mut player_id_str = String::new();
        for _ in 0..player_id_len {
            player_id_str.push(pd.next().ok_or(PacketDecodeError::UnexpectedEof)? as char);
        }
        let vehicle_id = u16::from_le_bytes([pd.next().ok_or(PacketDecodeError::UnexpectedEof)?, pd.next().ok_or(PacketDecodeError::UnexpectedEof)?]);
        let runtime_data = String::from_utf8(pd.collect::<Vec<u8>>()).map_err(|_| PacketDecodeError::InvalidString)?;

        let player_id: u64 = player_id_str.parse().map_err(|_| PacketDecodeError::InvalidNumber)?;

        Ok(Self {
            player_id,
            vehicle_id,
            runtime_data,
        })
    }

    pub fn to_raw(&self) -> Result<Vec<u8>, PacketEncodeError> {
        let player_id_str = self.player_id.to_string();
        let mut bytes = vec![player_id_str.len() as u8];
        bytes.extend_from_slice(&player_id_str.as_bytes());
        bytes.extend_from_slice(&self.vehicle_id.to_le_bytes());
        bytes.extend_from_slice(&self.runtime_data.as_bytes());
        Ok(bytes)
    }
}
