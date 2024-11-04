// TODO: Replace anyhow::Result<> with proper error handling everywhere
//       in this file.

use crate::*;

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs, UdpSocket};

/// A generic connection to be used anywhere it's needed.
/// Purely handles sending/receiving packets.
pub struct TcpConnection<T: PacketTrait> {
    packet_type: std::marker::PhantomData<T>,
    tcp: TcpStream,
    buf: Vec<u8>,
}

impl<T: PacketTrait> TcpConnection<T> {
    pub fn from_stream(tcp: TcpStream) -> Self {
        Self {
            packet_type: std::marker::PhantomData,
            tcp,
            buf: Vec::new(),
        }
    }

    fn read_to_buf(&mut self) -> anyhow::Result<usize> {
        // TODO: Figure out an appropriate length, maybe 4096 is too short
        let mut big_buf = [0u8; 4096];

        let read = match self.tcp.try_read(&mut big_buf) {
            Ok(n) => n,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    return Ok(0);
                }
                return Err(e.into());
            }
        };

        self.buf.extend_from_slice(&big_buf[..read]);

        Ok(read)
    }

    /// Assumes the socket is already readable.
    /// NOTE: This function WILL and SHOULD block until all bytes can be read from the buffer
    async fn read_bytes(&mut self, bytes: usize) -> anyhow::Result<Vec<u8>> {
        if bytes > self.buf.len() {
            let left_to_read = bytes - self.buf.len();
            let mut buf = vec![0u8; left_to_read];
            self.tcp.read_exact(&mut buf).await?;
            self.buf.append(&mut buf);
        }

        Ok(self.buf.drain(..bytes).collect::<Vec<u8>>())
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
        let packet_length =
            u32::from_le_bytes([header_raw[2], header_raw[3], header_raw[4], header_raw[5]]);
        Ok(PacketHeader {
            sig_a,
            sig_b,
            packet_length,
        })
    }

    /// TODO: Check if socket is readable?
    pub async fn wait_for_packet(&mut self) -> anyhow::Result<T> {
        let packet_header = self.read_packet_header().await?;
        let packet_data = self
            .read_bytes(packet_header.packet_length as usize)
            .await?;
        Ok(T::from_raw(
            packet_header.sig_a,
            packet_header.sig_b,
            packet_data,
        )?)
    }

    pub async fn try_read_packet(&mut self) -> anyhow::Result<Option<T>> {
        self.read_to_buf()?;

        // Read potential packet from self.buf now
        if self.buf.len() < 6 {
            return Ok(None);
        }
        let header_buf = &self.buf[..6];
        let sig_a = header_buf[0] as char;
        let sig_b = header_buf[1] as char;
        let packet_length =
            u32::from_le_bytes([header_buf[2], header_buf[3], header_buf[4], header_buf[5]]);

        // Once we know we have enough data to also read the packet data, we can start draining
        if self.buf.len() < packet_length as usize {
            return Ok(None);
        }
        self.buf.drain(..6);
        let data_buf = self
            .buf
            .drain(..(packet_length as usize))
            .collect::<Vec<u8>>();

        Ok(Some(T::from_raw(sig_a, sig_b, data_buf)?))
    }

    pub async fn write_packet(&mut self, packet: &T) -> anyhow::Result<()> {
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

    fn packet_from_buf(&self, addr: SocketAddr, buf: &[u8]) -> anyhow::Result<(T, SocketAddr)> {
        if buf.len() < 6 {
            return Err(ConnectionError::InvalidPacketSize.into());
        }

        let sig_a = buf[0] as char;
        let sig_b = buf[1] as char;
        let packet_length = u32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]);

        if buf.len() < 6 + packet_length as usize {
            return Err(ConnectionError::InvalidPacketSize.into());
        }

        Ok((T::from_raw(sig_a, sig_b, buf[6..].to_vec())?, addr))
    }

    pub async fn wait_for_packet(&mut self) -> anyhow::Result<(T, SocketAddr)> {
        let (bytes_read, addr) = self.udp_socket.recv_from(&mut self.recv_buf).await?;
        let buf = &self.recv_buf[..bytes_read];
        self.packet_from_buf(addr, buf)
    }

    pub fn try_read_packet(&mut self) -> anyhow::Result<Option<(T, SocketAddr)>> {
        match self.udp_socket.try_recv_from(&mut self.recv_buf) {
            Ok((bytes_read, addr)) => {
                let buf = &self.recv_buf[..bytes_read];
                Ok(Some(self.packet_from_buf(addr, buf)?))
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn write_bytes<A: ToSocketAddrs>(
        &mut self,
        target: &A,
        bytes: &[u8],
    ) -> anyhow::Result<()> {
        self.udp_socket.send_to(bytes, target).await?;
        Ok(())
    }

    pub async fn write_packet<A: ToSocketAddrs>(
        &mut self,
        target: A,
        packet: T,
    ) -> anyhow::Result<()> {
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
    pub async fn connect<A: ToSocketAddrs>(
        udp_socket: UdpSocket,
        target: A,
    ) -> anyhow::Result<Self> {
        udp_socket.connect(target).await?;
        trace!("peer_addr: {:?}", udp_socket.peer_addr());
        Ok(Self {
            packet_type: std::marker::PhantomData,
            udp_socket,
            recv_buf: vec![0u8; 65535],
        })
    }

    pub async fn wait_for_packet(&mut self) -> anyhow::Result<T> {
        let bytes_read = self.udp_socket.recv(&mut self.recv_buf).await?;
        let buf = &self.recv_buf[..bytes_read];

        if buf.len() < 6 {
            return Err(ConnectionError::InvalidPacketSize.into());
        }

        let sig_a = buf[0] as char;
        let sig_b = buf[1] as char;
        let packet_length = u32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]);

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
