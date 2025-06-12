pub mod tcp_server;

use std::{io, net::SocketAddr};
use crate::tcp_server::ServidorTcp;


fn main() -> Result<(), io::Error> {
    let server = match ServidorTcp::new(SocketAddr::from(([127, 0, 0, 1], 80))) {
        Ok(s) => s,
        Err(e) => panic!("Error en la conección al servidor \n{}", e),
    };

    println!("Servidor conectado a {}", server.addr);

    for stream in server.listener.incoming() {
        ServidorTcp::handle(stream?)?;
    }

    Ok(())
}
