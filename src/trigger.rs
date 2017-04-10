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

    pub fn enable(&mut self, source: &str) {
        self.socket.send(format!("ACQ:TRIG {}", source));
    }

    pub fn disable(&mut self) {
        self.enable("DISABLED");
    }

    pub fn get_state(&mut self) -> String {
        self.socket.send("ACQ:TRIG:STAT?");

        self.socket.receive()
    }

    pub fn set_delay(&mut self, delay: u8) {
        self.socket.send(format!("ACQ:TRIG:DLY {}", delay));
    }

    pub fn get_delay(&mut self) -> u16 {
        self.socket.send("ACQ:TRIG:DLY?");

        self.socket.receive()
            .parse()
            .unwrap()
    }

    pub fn set_delay_in_ns(&mut self, delay: u8) {
        self.socket.send(format!("ACQ:TRIG:DLY:NS {}", delay));
    }

    pub fn get_delay_in_ns(&mut self) -> String {
        self.socket.send("ACQ:TRIG:DLY:NS?");

        self.socket.receive()
    }

    pub fn set_gain(&mut self, gain: &str) {
        self.socket.send(format!("ACQ:SOUR1:GAIN {}", gain));
    }

    pub fn set_level(&mut self, level: u8) {
        self.socket.send(format!("ACQ:TRIG:LEV {}", level));
    }

    pub fn get_level(&mut self) -> String {
        self.socket.send("ACQ:TRIG:LEV?");

        self.socket.receive()
    }
}

#[cfg(test)]
mod test {
    macro_rules! trigger_assert {
        ($f:ident, $e:expr) => {
            let (rx, mut trigger) = create_trigger();

            trigger.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    #[test]
    fn test_enable() {
        let (rx, mut trigger) = create_trigger();

        trigger.enable("NOW");
        assert_eq!("ACQ:TRIG NOW\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_disable() {
        trigger_assert!(disable, "ACQ:TRIG DISABLED\r\n");
    }

    #[test]
    fn test_get_state() {
        let (_, mut trigger) = create_trigger();

        assert_eq!(trigger.get_state(), "WAIT");
    }

    #[test]
    fn test_set_delay() {
        let (rx, mut trigger) = create_trigger();

        trigger.set_delay(0);
        assert_eq!("ACQ:TRIG:DLY 0\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_delay() {
        let (_, mut trigger) = create_trigger();

        assert_eq!(trigger.get_delay(), 2314);
    }

    #[test]
    fn test_set_delay_in_ns() {
        let (rx, mut trigger) = create_trigger();

        trigger.set_delay_in_ns(0);
        assert_eq!("ACQ:TRIG:DLY:NS 0\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_delay_in_ns() {
        let (_, mut trigger) = create_trigger();

        assert_eq!(trigger.get_delay_in_ns().as_str(), "128ns");
    }

    #[test]
    fn test_set_gain() {
        let (rx, mut trigger) = create_trigger();

        trigger.set_gain("LV");
        assert_eq!("ACQ:SOUR1:GAIN LV\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_level() {
        let (rx, mut trigger) = create_trigger();

        trigger.set_level(0);
        assert_eq!("ACQ:TRIG:LEV 0\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_level() {
        let (_, mut trigger) = create_trigger();

        assert_eq!(trigger.get_level().as_str(), "123mV");
    }

    fn create_trigger() -> (::std::sync::mpsc::Receiver<String>, ::trigger::Trigger) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(
            format!("{}", addr.ip()).as_str(),
            addr.port()
        );

        (rx, ::trigger::Trigger::new(socket))
    }
}
