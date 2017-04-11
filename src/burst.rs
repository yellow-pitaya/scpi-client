use socket::Socket;

pub struct Burst {
    socket: Socket,
}

impl Burst {
    pub fn new(socket: Socket) -> Self {
        Burst {
            socket: socket,
        }
    }

    pub fn enable(&mut self, source: u8) {
        self.socket.send(format!("SOUR{}:BURS:STAT ON", source));
    }

    pub fn disable(&mut self, source: u8) {
        self.socket.send(format!("SOUR{}:BURS:STAT OFF", source));
    }

    pub fn set_count(&mut self, source: u8, count: u32) {
        self.socket.send(format!("SOUR{}:BURS:NCYC {}", source, count));
    }

    pub fn set_repetitions(&mut self, source: u8, repetitions: u32) {
        self.socket.send(format!("SOUR{}:BURS:NOR {}", source, repetitions));
    }

    pub fn set_period(&mut self, source: u8, period: u32) {
        self.socket.send(format!("SOUR{}:BURS:INT:PER {}", source, period));
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_enable() {
        let (rx, mut burst) = create_burst();

        burst.enable(1);
        assert_eq!("SOUR1:BURS:STAT ON\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_disable() {
        let (rx, mut burst) = create_burst();

        burst.disable(1);
        assert_eq!("SOUR1:BURS:STAT OFF\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_count() {
        let (rx, mut burst) = create_burst();

        burst.set_count(1, 3);
        assert_eq!("SOUR1:BURS:NCYC 3\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_repetitions() {
        let (rx, mut burst) = create_burst();

        burst.set_repetitions(1, 5);
        assert_eq!("SOUR1:BURS:NOR 5\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_period() {
        let (rx, mut burst) = create_burst();

        burst.set_period(1, 1_000_000);
        assert_eq!("SOUR1:BURS:INT:PER 1000000\r\n", rx.recv().unwrap());
    }

    fn create_burst() -> (::std::sync::mpsc::Receiver<String>, ::burst::Burst) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(
            format!("{}", addr.ip()).as_str(),
            addr.port()
        );

        (rx, ::burst::Burst::new(socket))
    }
}
