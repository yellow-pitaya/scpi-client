use std::io::prelude::*;

#[derive(Clone)]
pub struct Socket {
    addr: String,
}

impl Socket {
    pub fn new(addr: String) -> Self
    {
        Socket {
            addr,
        }
    }

    pub fn send<D>(&self, command: D) -> Option<String>
        where D: std::fmt::Display
    {
        let mut stream = match std::net::TcpStream::connect(self.addr.clone()) {
            Ok(stream) => stream,
            Err(_) => panic!("Unable to connect"),
        };

        log::info!("> {}", command);

        let message = format!("{}\r\n", command);
        stream.write_all(message.as_bytes())
            .unwrap();

        if message.contains('?') {
            Some(self.receive(stream))
        }
        else {
            None
        }
    }

    fn receive(&self, stream: std::net::TcpStream) -> String {
        let mut message = String::new();
        let mut reader = std::io::BufReader::new(stream);

        reader.read_line(&mut message)
            .unwrap();

        let message = message.trim_end_matches("\r\n");

        log::debug!("< {}", message);

        message.into()
    }
}
