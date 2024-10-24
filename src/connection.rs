// TODO: Replace anyhow::Result<> with proper error handling everywhere
//       in this file.

use crate::*;

use std::net::SocketAddr;
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
    pub fn from_stream(tcp: TcpStream) -> Self {
        Self {
            packet_type: std::marker::PhantomData,

            tcp,
        }
    }

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
    pub async fn wait_for_packet(&mut self) -> anyhow::Result<T> {
        let packet_header = self.read_packet_header().await?;
        let packet_data = self.read_bytes(packet_header.packet_length as usize).await?;
        Ok(T::from_raw(packet_header.sig_a, packet_header.sig_b, packet_data)?)
    }

    pub async fn try_read_packet(&mut self) -> anyhow::Result<Option<T>> {
        let mut header_buf = [0u8; 6];
        let read = self.tcp.peek(&mut header_buf).await?;
        if read != 6 { return Ok(None); }

        let sig_a = header_buf[0] as char;
        let sig_b = header_buf[1] as char;
        let packet_length = u32::from_le_bytes([header_buf[2], header_buf[3], header_buf[4], header_buf[5]]);

        let mut data_buf = vec![0u8; packet_length as usize];
        let read = self.tcp.peek(&mut data_buf).await?;
        if read != packet_length as usize { return Ok(None); }

        // We are done, now we must consume the data from the actual queue
        self.tcp.read(&mut header_buf).await?;
        self.tcp.read(&mut data_buf).await?;

        Ok(Some(T::from_raw(sig_a, sig_b, data_buf)?))
    }

    pub async fn write_packet(&mut self, packet: T) -> anyhow::Result<()> {
        let (sig_a, sig_b, mut raw) = packet.to_raw()?;
        let mut bytes = Vec::new(); // TODO: with_capacity for less allocations and more performance
        bytes.push(sig_a as u8);
        bytes.push(sig_b as u8);
        bytes.extend_from_slice(&mut u32::to_le_bytes(raw.len() as u32));
        bytes.append(&mut raw);
        self.write_bytes(&bytes).await?;
        Ok(())
    }
}

pub struct UdpListener<T: PacketTrait> {
    packet_type: std::marker::PhantomData<T>,

    udp_socket: Arc<UdpSocket>,
    recv_buf: Vec<u8>,
}

impl<T: PacketTrait> UdpListener<T> {
    pub async fn bind<A: tokio::net::ToSocketAddrs>(addr: A) -> tokio::io::Result<Self> {
        Ok(Self {
            packet_type: std::marker::PhantomData,
            udp_socket: Arc::new(UdpSocket::bind(addr).await?),
            recv_buf: vec![0u8; 65535],
        })
    }

    pub async fn wait_for_packet(&mut self) -> anyhow::Result<(T, SocketAddr)> {
        let (bytes_read, addr) = self.udp_socket.recv_from(&mut self.recv_buf).await?;
        trace!("bytes read: {} (from {})", bytes_read, addr);
        let buf = &self.recv_buf[..bytes_read];

        if buf.len() < 6 {
            return Err(ConnectionError::InvalidPacketSize.into());
        }

        let sig_a = buf[0] as char;
        let sig_b = buf[1] as char;
        let packet_length = u32::from_le_bytes([
                buf[2],
                buf[3],
                buf[4],
                buf[5],
            ]);
        trace!("packet_length: {}", packet_length);

        if buf.len() < 6 + packet_length as usize {
            return Err(ConnectionError::InvalidPacketSize.into());
        }

        Ok((T::from_raw(sig_a, sig_b, buf[6..].to_vec())?, addr))
    }

    pub async fn write_bytes<A: ToSocketAddrs>(&mut self, target: &A, bytes: &[u8]) -> anyhow::Result<()> {
        self.udp_socket.send_to(bytes, target).await?;
        Ok(())
    }

    pub async fn write_packet<A: ToSocketAddrs>(&mut self, target: A, packet: T) -> anyhow::Result<()> {
        let (sig_a, sig_b, mut raw) = packet.to_raw()?;
        let mut bytes = Vec::new(); // TODO: with_capacity for less allocations and more performance
        bytes.push(sig_a as u8);
        bytes.push(sig_b as u8);
        bytes.extend_from_slice(&mut u32::to_le_bytes(raw.len() as u32));
        bytes.append(&mut raw);
        self.write_bytes(&target, &bytes).await?;
        Ok(())
    }
}

/// A generic connection to be used anywhere it's needed.
/// Purely handles sending/receiving packets.
pub struct UdpClient<T: PacketTrait> {
    packet_type: std::marker::PhantomData<T>,

    udp_socket: UdpSocket,
    recv_buf: Vec<u8>,
}

impl<T: PacketTrait> UdpClient<T> {
    pub async fn connect<A: ToSocketAddrs>(mut udp_socket: UdpSocket, target: A) -> anyhow::Result<Self> {
        udp_socket.connect(target).await?;
        Ok(Self {
            packet_type: std::marker::PhantomData,
            udp_socket,
            recv_buf: vec![0u8; 65535],
        })
    }

    pub async fn wait_for_packet(&mut self) -> anyhow::Result<T> {
        let bytes_read = self.udp_socket.recv(&mut self.recv_buf).await?;
        trace!("bytes read: {}", bytes_read);
        let buf = &self.recv_buf[..bytes_read];

        if buf.len() < 6 {
            return Err(ConnectionError::InvalidPacketSize.into());
        }

        let sig_a = buf[0] as char;
        let sig_b = buf[1] as char;
        let packet_length = u32::from_le_bytes([
                buf[2],
                buf[3],
                buf[4],
                buf[5],
            ]);
        trace!("packet_length: {}", packet_length);

        if buf.len() < 6 + packet_length as usize {
            return Err(ConnectionError::InvalidPacketSize.into());
        }

        Ok(T::from_raw(sig_a, sig_b, buf[6..].to_vec())?)
    }

    pub async fn write_bytes(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        self.udp_socket.send(bytes).await?;
        Ok(())
    }

    pub async fn write_packet(&mut self, packet: T) -> anyhow::Result<()> {
        let (sig_a, sig_b, mut raw) = packet.to_raw()?;
        let mut bytes = Vec::new(); // TODO: with_capacity for less allocations and more performance
        bytes.push(sig_a as u8);
        bytes.push(sig_b as u8);
        bytes.extend_from_slice(&mut u32::to_le_bytes(raw.len() as u32));
        bytes.append(&mut raw);
        self.write_bytes(&bytes).await?;
        Ok(())
    }
}
