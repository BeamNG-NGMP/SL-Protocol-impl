// TODO: Replace anyhow::Result<> with proper error handling everywhere
//       in this file.

use crate::*;
use crate::server_launcher::*;

use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{
    TcpStream,
    UdpSocket,
    ToSocketAddrs,
};

/// A generic connection to be used anywhere it's needed.
/// Purely handles sending/receiving packets.
pub struct TcpConnection<T: PacketTrait> {
    packet_type: std::marker::PhantomData<T>,

    tcp: TcpStream,
}

impl<T: PacketTrait> TcpConnection<T> {
    /// Assumes the socket is already readable.
    async fn read_bytes(&mut self, bytes: usize) -> anyhow::Result<Vec<u8>> {
        let mut data: Vec<u8> = (0..bytes).into_iter().map(|_| 0u8).collect();

        self.tcp.read_exact(&mut data).await?;
        Ok(data)
    }

    async fn write_bytes(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        let mut bytes = std::io::Cursor::new(bytes);
        self.tcp.write_all_buf(&mut bytes).await?;
        Ok(())
    }

    async fn read_packet_header(&mut self) -> anyhow::Result<PacketHeader> {
        let header_raw = self.read_bytes(6).await?;
        let sig_a = header_raw[0] as char;
        let sig_b = header_raw[1] as char;
        let packet_length = u32::from_le_bytes([header_raw[2], header_raw[3], header_raw[4], header_raw[5]]);
        Ok(PacketHeader {
            sig_a,
            sig_b,
            packet_length,
        })
    }

    /// TODO: Check if socket is readable?
    pub async fn read_packet(&mut self) -> anyhow::Result<T> {
        let packet_header = self.read_packet_header().await?;
        let packet_data = self.read_bytes(packet_header.packet_length as usize).await?;
        Ok(T::from_raw(packet_header.sig_a, packet_header.sig_b, packet_data)?)
    }

    pub async fn write_packet(&mut self, packet: T) -> anyhow::Result<()> {
        let (sig_a, sig_b, raw) = packet.to_raw()?;
        self.write_bytes(&[sig_a as u8, sig_b as u8]).await?;
        self.write_bytes(&u32::to_le_bytes(raw.len() as u32)).await?;
        self.write_bytes(&raw).await?;

        Ok(())
    }
}

pub struct UdpListener<T: PacketTrait> {
    packet_type: std::marker::PhantomData<T>,

    udp_socket: Arc<UdpSocket>,

    udp_buffer: Vec<u8>,
}

impl<T: PacketTrait> UdpListener<T> {
    pub async fn bind<A: tokio::net::ToSocketAddrs>(addr: A) -> tokio::io::Result<Self> {
        Ok(Self {
            packet_type: std::marker::PhantomData,
            udp_socket: Arc::new(UdpSocket::bind(addr).await?),
            udp_buffer: Vec::new(),
        })
    }

    /// Assumes there is enough data in the buffer to read a packet header.
    fn bufread_packet_header(&mut self) -> PacketHeader {
        let mut raw = self.udp_buffer.drain(0..6);
        let sig_a = raw.next().unwrap() as char;
        let sig_b = raw.next().unwrap() as char;
        let packet_length = u32::from_le_bytes([
                raw.next().unwrap(),
                raw.next().unwrap(),
                raw.next().unwrap(),
                raw.next().unwrap(),
            ]);
        PacketHeader {
            sig_a,
            sig_b,
            packet_length,
        }
    }

    pub async fn read_packets(&mut self) -> anyhow::Result<Vec<T>> {
        self.udp_socket.recv_buf(&mut self.udp_buffer).await?;

        let mut packets = Vec::new();

        while self.udp_buffer.len() >= 6 {
            let packet_header = self.bufread_packet_header();
            if self.udp_buffer.len() < packet_header.packet_length as usize {
                // Not enough data in the buffer to read the current buffer!
                // Now we have to put our packet header data back :(
                let raw = packet_header.packet_length.to_le_bytes();
                self.udp_buffer.insert(0, raw[3]);
                self.udp_buffer.insert(0, raw[2]);
                self.udp_buffer.insert(0, raw[1]);
                self.udp_buffer.insert(0, raw[0]);
                self.udp_buffer.insert(0, packet_header.sig_b as u8);
                self.udp_buffer.insert(0, packet_header.sig_a as u8);
            } else {
                // We have enough data to read a packet :)
                let data: Vec<u8> = self.udp_buffer.drain(0..(packet_header.packet_length as usize)).collect();
                match T::from_raw(packet_header.sig_a, packet_header.sig_b, data) {
                    Ok(packet) => packets.push(packet),
                    Err(e) => error!("UDP packet failed to parse! {e}"),
                }
            }
        }

        Ok(packets)
    }

    pub async fn write_bytes<A: ToSocketAddrs>(&mut self, target: &A, bytes: &[u8]) -> anyhow::Result<()> {
        self.udp_socket.send_to(bytes, target).await?;
        Ok(())
    }

    pub async fn write_packet<A: ToSocketAddrs>(&mut self, target: A, packet: T) -> anyhow::Result<()> {
        let (sig_a, sig_b, raw) = packet.to_raw()?;
        self.write_bytes(&target, &[sig_a as u8, sig_b as u8]).await?;
        self.write_bytes(&target, &u32::to_le_bytes(raw.len() as u32)).await?;
        self.write_bytes(&target, &raw).await?;
        Ok(())
    }
}

/// A generic connection to be used anywhere it's needed.
/// Purely handles sending/receiving packets.
pub struct UdpClient<T: PacketTrait> {
    packet_type: std::marker::PhantomData<T>,

    udp_socket: UdpSocket,

    udp_buffer: Vec<u8>,
}

impl<T: PacketTrait> UdpClient<T> {
    pub async fn connect<A: ToSocketAddrs>(target: A) -> anyhow::Result<Self> {
        let udp_socket = UdpSocket::bind("0.0.0.0:0").await?;
        udp_socket.connect(target).await?;
        Ok(Self {
            packet_type: std::marker::PhantomData,
            udp_socket,
            udp_buffer: Vec::new(),
        })
    }

    /// Assumes there is enough data in the buffer to read a packet header.
    fn bufread_packet_header(&mut self) -> PacketHeader {
        let mut raw = self.udp_buffer.drain(0..6);
        let sig_a = raw.next().unwrap() as char;
        let sig_b = raw.next().unwrap() as char;
        let packet_length = u32::from_le_bytes([
                raw.next().unwrap(),
                raw.next().unwrap(),
                raw.next().unwrap(),
                raw.next().unwrap(),
            ]);
        PacketHeader {
            sig_a,
            sig_b,
            packet_length,
        }
    }

    pub async fn read_packets(&mut self) -> anyhow::Result<Vec<T>> {
        self.udp_socket.recv_buf(&mut self.udp_buffer).await?;

        let mut packets = Vec::new();

        while self.udp_buffer.len() >= 6 {
            let packet_header = self.bufread_packet_header();
            if self.udp_buffer.len() < packet_header.packet_length as usize {
                // Not enough data in the buffer to read the current buffer!
                // Now we have to put our packet header data back :(
                let raw = packet_header.packet_length.to_le_bytes();
                self.udp_buffer.insert(0, raw[3]);
                self.udp_buffer.insert(0, raw[2]);
                self.udp_buffer.insert(0, raw[1]);
                self.udp_buffer.insert(0, raw[0]);
                self.udp_buffer.insert(0, packet_header.sig_b as u8);
                self.udp_buffer.insert(0, packet_header.sig_a as u8);
            } else {
                // We have enough data to read a packet :)
                let data: Vec<u8> = self.udp_buffer.drain(0..(packet_header.packet_length as usize)).collect();
                match T::from_raw(packet_header.sig_a, packet_header.sig_b, data) {
                    Ok(packet) => packets.push(packet),
                    Err(e) => error!("UDP packet failed to parse! {e}"),
                }
            }
        }

        Ok(packets)
    }

    pub async fn write_bytes(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        self.udp_socket.send(bytes).await?;
        Ok(())
    }

    pub async fn write_packet(&mut self, packet: T) -> anyhow::Result<()> {
        let (sig_a, sig_b, raw) = packet.to_raw()?;
        self.write_bytes(&[sig_a as u8, sig_b as u8]).await?;
        self.write_bytes(&u32::to_le_bytes(raw.len() as u32)).await?;
        self.write_bytes(&raw).await?;
        Ok(())
    }
}
