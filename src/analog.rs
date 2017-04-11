use socket::Socket;

pub struct Analog {
    socket: Socket,
}

impl Analog {
    pub fn new(socket: Socket) -> Self {
        Analog {
            socket: socket,
        }
    }

    pub fn set_value(&mut self, pin: &str, value: f32) {
        self.socket.send(format!("ANALOG:PIN {},{}", pin, value));
    }

    pub fn get_value(&mut self, pin: &str) -> f32 {
        self.socket.send(format!("ANALOG:PIN? {}", pin));

        self.socket.receive()
            .parse()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_set_value() {
        let (rx, mut analog) = create_analog();

        analog.set_value("AOUT2", 1.34);
        assert_eq!("ANALOG:PIN AOUT2,1.34\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_value() {
        let (_, mut analog) = create_analog();

        assert_eq!(analog.get_value("AOUT2"), 1.34);
    }

    fn create_analog() -> (::std::sync::mpsc::Receiver<String>, ::analog::Analog) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(
            format!("{}", addr.ip()).as_str(),
            addr.port()
        );

        (rx, ::analog::Analog::new(socket))
    }
}
