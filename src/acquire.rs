use socket::Socket;

pub struct Acquire {
    socket: Socket,
    started: bool,
}

impl Acquire {
    pub fn new(socket: Socket) -> Self {
        Acquire {
            socket: socket,
            started: false,
        }
    }

    pub fn start(&mut self) {
        self.socket.send("ACQ:START");
        self.started = true;
    }

    pub fn stop(&mut self) {
        self.socket.send("ACQ:STOP");
        self.started = false;
    }

    pub fn is_started(&self) -> bool {
        self.started
    }

    pub fn reset(&mut self) {
        self.socket.send("ACQ:RST");
    }

    pub fn set_units(&mut self, unit: &str) {
        self.socket.send(format!("ACQ:DATA:UNITS {}", unit));
    }

    pub fn set_decimation(&mut self, decimation: u8) {
        self.socket.send(format!("ACQ:DEC {}", decimation));
    }

    pub fn get_decimation(&mut self) -> u8 {
        self.socket.send("ACQ:DEC?");

        self.socket.receive()
            .parse()
            .unwrap()
    }

    pub fn get_data(&mut self) -> String {
        self.socket.send("ACQ:SOUR1:DATA?");

        self.socket.receive()
    }
}

#[cfg(test)]
mod test {
    macro_rules! acquire_assert {
        ($f:ident, $e:expr) => {
            let (rx, mut acquire) = create_acquire();

            acquire.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    fn create_acquire() -> (::std::sync::mpsc::Receiver<String>, ::acquire::Acquire) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(
            format!("{}", addr.ip()).as_str(),
            addr.port()
        );

        (rx, ::acquire::Acquire::new(socket))
    }

    #[test]
    fn test_start() {
        acquire_assert!(start, "ACQ:START\r\n");
    }

    #[test]
    fn test_stop() {
        acquire_assert!(stop, "ACQ:STOP\r\n");
    }

    #[test]
    fn test_is_started() {
        let (_, mut acquire) = create_acquire();

        assert_eq!(acquire.is_started(), false);
        acquire.start();
        assert_eq!(acquire.is_started(), true);
        acquire.stop();
        assert_eq!(acquire.is_started(), false);
    }

    #[test]
    fn test_reset() {
        acquire_assert!(reset, "ACQ:RST\r\n");
    }

    #[test]
    fn test_set_units() {
        let (rx, mut acquire) = create_acquire();

        acquire.set_units("VOLTS");
        assert_eq!("ACQ:DATA:UNITS VOLTS\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_decimation() {
        let (rx, mut acquire) = create_acquire();

        acquire.set_decimation(1);
        assert_eq!("ACQ:DEC 1\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_decimation() {
        let (_, mut acquire) = create_acquire();

        assert_eq!(acquire.get_decimation(), 1);
    }

    #[test]
    fn test_data() {
        let (_, mut acquire) = create_acquire();

        assert_eq!(acquire.get_data(), String::from("{1.2,3.2,-1.2}"));
    }
}
