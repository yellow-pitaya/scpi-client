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

    pub fn enable(&mut self) {
        self.socket.send("SOUR1:BURS:STAT ON");
    }

    pub fn disable(&mut self) {
        self.socket.send("SOUR1:BURS:STAT OFF");
    }

    pub fn set_count(&mut self, count: u32) {
        self.socket.send(format!("SOUR1:BURS:NCYC {}", count));
    }

    pub fn set_repetitions(&mut self, repetitions: u32) {
        self.socket.send(format!("SOUR1:BURS:NOR {}", repetitions));
    }

    pub fn set_period(&mut self, period: u32) {
        self.socket.send(format!("SOUR1:BURS:INT:PER {}", period));
    }
}

#[cfg(test)]
mod test {
    macro_rules! burst_assert {
        ($f:ident, $e:expr) => {
            let (rx, mut burst) = create_burst();

            burst.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    #[test]
    fn test_enable() {
        burst_assert!(enable, "SOUR1:BURS:STAT ON\r\n");
    }

    #[test]
    fn test_disable() {
        burst_assert!(disable, "SOUR1:BURS:STAT OFF\r\n");
    }

    #[test]
    fn test_set_count() {
        let (rx, mut burst) = create_burst();

        burst.set_count(3);
        assert_eq!("SOUR1:BURS:NCYC 3\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_repetitions() {
        let (rx, mut burst) = create_burst();

        burst.set_repetitions(5);
        assert_eq!("SOUR1:BURS:NOR 5\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_period() {
        let (rx, mut burst) = create_burst();

        burst.set_period(1_000_000);
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
