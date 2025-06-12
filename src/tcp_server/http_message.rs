use std::{
    io::{self,BufReader, BufRead}, 
    net::TcpStream
};

pub struct HttpMessage {
    first: String,
    rest: Option<Vec<String>>
}

enum MessagesTypes {
    Peticion, 
    Respuesta,
    NoReconocido
}

impl HttpMessage {
    pub fn new(stream: TcpStream) -> Result<Self, io::Error> {
        let mut reader = BufReader::new(stream);
        let mut primera_linea = String::new();

        reader.read_line(&mut primera_linea)?;

        let mut resto: Vec<String> = Vec::new();

        for resultado in reader.lines() {
            let line = resultado?;
            if line.is_empty(){
                break;
            } else {
                resto.push(line);
            }
        }

        if resto.is_empty(){
            Ok(Self {
                first: primera_linea,
                rest: None,
            })
           
        } else {
            Ok(Self {
                first: primera_linea,
                rest: Some(resto),
            })
        }

    }

    pub fn transfrom(self) -> MessagesTypes {
        
    }
}
