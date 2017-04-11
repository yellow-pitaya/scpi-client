use socket::Socket;

pub enum Source {
    DISABLED,
    NOW,
    CH1_PE,
    CH1_NE,
    CH2_PE,
    CH2_NE,
    EXT_PE,
    EXT_NE,
    AWG_PE,
    AWG_NE,
}

impl ::std::fmt::Display for Source {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Source::DISABLED => "DISABLED",
            &Source::NOW => "NOW",
            &Source::CH1_PE => "CH1_PE",
            &Source::CH1_NE => "CH1_NE",
            &Source::CH2_PE => "CH2_PE",
            &Source::CH2_NE => "CH2_NE",
            &Source::EXT_PE => "EXT_PE",
            &Source::EXT_NE => "EXT_NE",
            &Source::AWG_PE => "AWG_PE",
            &Source::AWG_NE => "AWG_NE",
        };

        write!(f, "{}", display)
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    WAIT,
    TD,
    UNKNOW,
}

impl ::std::convert::From<String> for State {
    fn from(s: String) -> Self {
        match s.as_str() {
            "WAIT" => State::WAIT,
            "TD" => State::TD,
            _ => State::UNKNOW,
        }
    }
}

pub struct Trigger {
    socket: Socket,
}

impl Trigger {
    pub fn new(socket: Socket) -> Self {
        Trigger {
            socket: socket,
        }
    }

    pub fn enable(&mut self, source: Source) {
        self.socket.send(format!("ACQ:TRIG {}", source));
    }

    pub fn disable(&mut self) {
        self.enable(Source::DISABLED);
    }

    pub fn get_state(&mut self) -> State {
        self.socket.send("ACQ:TRIG:STAT?");

        self.socket.receive()
            .into()
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

        trigger.enable(::trigger::Source::NOW);
        assert_eq!("ACQ:TRIG NOW\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_disable() {
        trigger_assert!(disable, "ACQ:TRIG DISABLED\r\n");
    }

    #[test]
    fn test_get_state() {
        let (_, mut trigger) = create_trigger();

        assert_eq!(trigger.get_state(), ::trigger::State::WAIT);
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
        let socket = ::socket::Socket::new(addr);

        (rx, ::trigger::Trigger::new(socket))
    }
}
