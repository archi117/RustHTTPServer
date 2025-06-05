use std::{
    io::{self, BufRead, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
};

struct ServidorTcp {
    addr: SocketAddr,
    listener: TcpListener,
}

struct HttpMessage {
    method: String,
    headers: Vec<String>,
    body: Option<Vec<String>>,
}

impl ServidorTcp {
    fn new(socket_addr: SocketAddr) -> Result<Self, io::Error> {
        let new_listener = TcpListener::bind(socket_addr)?;
        Ok(Self {
            addr: socket_addr,
            listener: new_listener,
        })
    }

    fn handle(stream: TcpStream) -> Result<(), io::Error> {
        let mensaje = HttpMessage::new(stream)?;
        Ok(())
    }
}

impl HttpMessage {
    fn new(stream: TcpStream) -> Result<Self, io::Error> {
        let mut buf_reader = BufReader::new(&stream);
        let mut line: String;

        let mut cabezales: Vec<String>;
        let mut cuerpo: Option<Vec<String>>;

        let metodo = buf_reader.read_line(&mut line)?;

        for l in buf_reader.lines()? {
            if l.is_empty() {
                break;
            }

            cabezales.push(l)
        }

        if buf_reader.is_empty() {
            cuerpo = None;
        } else {
            for l in buf_reader.lines()? {
                if l.is_empty() {
                    break;
                }

                cuerpo.push(l)
            }
        }

        Ok(Self {
            method: metodo,
            headers: cabezales,
            body: cuerpo,
        })
    }
}

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
