use socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Clone)]
pub struct Trigger {
    socket: ::std::cell::RefCell<Socket>,
}

impl Trigger {
    pub fn new(socket: Socket) -> Self {
        Trigger {
            socket: ::std::cell::RefCell::new(socket),
        }
    }

    /**
     * Trigger immediately or set trigger source & edge.
     */
    pub fn enable(&self, source: Source) {
        self.send(format!("ACQ:TRIG {}", source));
    }

    /**
     * Disable triggering.
     */
    pub fn disable(&self) {
        self.enable(Source::DISABLED);
    }

    /**
     *  Get trigger status.
     *
     *  If DISABLED -> TD else WAIT.
     */
    pub fn get_state(&self) -> State {
        self.send("ACQ:TRIG:STAT?");

        self.receive()
            .into()
    }

    /**
     * Set trigger delay in samples.
     */
    pub fn set_delay(&self, delay: u16) {
        self.send(format!("ACQ:TRIG:DLY {}", delay));
    }

    /**
     * Get trigger delay in samples.
     */
    pub fn get_delay(&self) -> u16 {
        self.send("ACQ:TRIG:DLY?");

        self.receive()
            .parse()
            .unwrap()
    }

    /**
     * Set trigger delay in ns.
     */
    pub fn set_delay_in_ns(&self, delay: u8) {
        self.send(format!("ACQ:TRIG:DLY:NS {}", delay));
    }

    /**
     * Get trigger delay in ns.
     */
    pub fn get_delay_in_ns(&self) -> u8 {
        self.send("ACQ:TRIG:DLY:NS?");

        self.receive()
            .replace("ns", "")
            .parse()
            .unwrap()
    }

    /**
     * Set trigger level in mV.
     */
    pub fn set_level(&self, level: u8) {
        self.send(format!("ACQ:TRIG:LEV {}", level));
    }

    /**
     * Get trigger level in mV.
     */
    pub fn get_level(&self) -> u8 {
        self.send("ACQ:TRIG:LEV?");

        self.receive()
            .replace("mV", "")
            .parse()
            .unwrap()
    }

    fn send<D>(&self, message: D)
        where D: ::std::fmt::Display
    {
        let mut socket = self.socket.borrow_mut();

        socket.send(message);
    }

    fn receive(&self) -> String {
        let mut socket = self.socket.borrow_mut();

        socket.receive()
    }
}

#[cfg(test)]
mod test {
    macro_rules! trigger_assert {
        ($f:ident, $e:expr) => {
            let (rx, trigger) = create_trigger();

            trigger.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    #[test]
    fn test_enable() {
        let (rx, trigger) = create_trigger();

        trigger.enable(::trigger::Source::NOW);
        assert_eq!("ACQ:TRIG NOW\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_disable() {
        trigger_assert!(disable, "ACQ:TRIG DISABLED\r\n");
    }

    #[test]
    fn test_get_state() {
        let (_, trigger) = create_trigger();

        assert_eq!(trigger.get_state(), ::trigger::State::WAIT);
    }

    #[test]
    fn test_set_delay() {
        let (rx, trigger) = create_trigger();

        trigger.set_delay(0);
        assert_eq!("ACQ:TRIG:DLY 0\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_delay() {
        let (_, trigger) = create_trigger();

        assert_eq!(trigger.get_delay(), 2314);
    }

    #[test]
    fn test_set_delay_in_ns() {
        let (rx, trigger) = create_trigger();

        trigger.set_delay_in_ns(0);
        assert_eq!("ACQ:TRIG:DLY:NS 0\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_delay_in_ns() {
        let (_, trigger) = create_trigger();

        assert_eq!(trigger.get_delay_in_ns(), 128);
    }

    #[test]
    fn test_set_level() {
        let (rx, trigger) = create_trigger();

        trigger.set_level(0);
        assert_eq!("ACQ:TRIG:LEV 0\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_level() {
        let (_, trigger) = create_trigger();

        assert_eq!(trigger.get_level(), 123);
    }

    fn create_trigger() -> (::std::sync::mpsc::Receiver<String>, ::trigger::Trigger) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::trigger::Trigger::new(socket))
    }
}
