use std::io::prelude::*;

pub struct Socket {
    socket: ::std::net::TcpStream,
}

impl Socket {
    pub fn new<S>(addr: S) -> Self where S: ::std::net::ToSocketAddrs {
        let socket = match ::std::net::TcpStream::connect(addr) {
            Ok(socket) => socket,
            Err(_) => panic!("Unable to connect"),
        };

        Socket {
            socket: socket,
        }
    }

    pub fn send<D>(&mut self, command: D) where D: ::std::fmt::Display {
        info!("> {}", command);

        self.socket.write(format!("{}\r\n", command).as_bytes())
            .unwrap();
    }

    pub fn receive(&mut self) -> String {
        let mut message = String::new();
        let mut reader = ::std::io::BufReader::new(self.socket.try_clone().unwrap());

        reader.read_line(&mut message)
            .unwrap();

        let message = message.trim_right_matches("\r\n");

        debug!("< {}", message);

        message.into()
    }
}

impl Clone for Socket {
    fn clone(&self) -> Self {
        Socket {
            socket: self.socket.try_clone().unwrap(),
        }
    }
}
