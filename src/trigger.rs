use socket::Socket;

pub struct Trigger {
    socket: Socket,
}

impl Trigger {
    pub fn new(socket: Socket) -> Self {
        Trigger {
            socket: socket,
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.socket.send(format!("ACQ:TRIG:LEV {}", level));
    }

    pub fn enable(&mut self, source: &str) {
        self.socket.send(format!("ACQ:TRIG {}", source));
    }

    pub fn set_delay(&mut self, delay: u8) {
        self.socket.send(format!("ACQ:TRIG:DLY {}", delay));
    }
}

#[cfg(test)]
mod test {
    fn create_trigger() -> (::std::sync::mpsc::Receiver<String>, ::trigger::Trigger) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(
            format!("{}", addr.ip()).as_str(),
            addr.port()
        );

        (rx, ::trigger::Trigger::new(socket))
    }

    #[test]
    fn test_set_level() {
        let (rx, mut trigger) = create_trigger();

        trigger.set_level(0);
        assert_eq!("ACQ:TRIG:LEV 0\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_enable() {
        let (rx, mut trigger) = create_trigger();

        trigger.enable("NOW");
        assert_eq!("ACQ:TRIG NOW\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_delay() {
        let (rx, mut trigger) = create_trigger();

        trigger.set_delay(0);
        assert_eq!("ACQ:TRIG:DLY 0\r\n", rx.recv().unwrap());
    }
}
