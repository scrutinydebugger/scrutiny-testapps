use std::io;
use std::net::{SocketAddr, UdpSocket};

pub struct UdpListener {
    socket: UdpSocket,
    sender: Option<SocketAddr>,
}

impl UdpListener {
    pub fn new(port: u16) -> io::Result<Self> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{port}"))?;
        socket.set_nonblocking(true)?;
        println!("UDP comm on port {port}");
        Ok(Self { socket, sender: None })
    }

    /// Reads one datagram into `buf`. Returns byte count, or 0 if no data is available.
    pub fn poll(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.socket.recv_from(buf) {
            Ok((n, addr)) => {
                self.sender = Some(addr);
                Ok(n)
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(0),
            Err(e) => Err(e),
        }
    }

    /// Sends `buf` back to the last known sender. No-op if no sender has been seen yet.
    pub fn send(&mut self, buf: &[u8]) -> io::Result<()> {
        if let Some(addr) = self.sender {
            self.socket.send_to(buf, addr)?;
        }
        Ok(())
    }
}
