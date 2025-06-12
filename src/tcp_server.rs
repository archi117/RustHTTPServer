use std::{
    io,
    net::{SocketAddr, TcpListener, TcpStream}
};

pub mod http_message;
pub mod request;
pub mod response;

use self::http_message::HttpMessage;

pub struct ServidorTcp {
    pub addr: SocketAddr,
    pub listener: TcpListener,
}

impl ServidorTcp {
    pub fn new(socket_addr: SocketAddr) -> Result<Self, io::Error> {
        let new_listener = TcpListener::bind(socket_addr)?;
        Ok(Self {
            addr: socket_addr,
            listener: new_listener,
        })
    }

    pub fn handle(stream: TcpStream) -> Result<(), io::Error> {
        let mensaje = HttpMessage::new(stream)?;
        Ok(())
    }
}
