use Module;
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

impl ::std::convert::Into<String> for Source {
    fn into(self) -> String {
        let s = match self {
            Source::DISABLED => "DISABLED",
            Source::NOW => "NOW",
            Source::CH1_PE => "CH1_PE",
            Source::CH1_NE => "CH1_NE",
            Source::CH2_PE => "CH2_PE",
            Source::CH2_NE => "CH2_NE",
            Source::EXT_PE => "EXT_PE",
            Source::EXT_NE => "EXT_NE",
            Source::AWG_PE => "AWG_PE",
            Source::AWG_NE => "AWG_NE",
        };

        String::from(s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    WAIT,
    TD,
}

impl ::std::str::FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WAIT" => Ok(State::WAIT),
            "TD" => Ok(State::TD),
            state => Err(format!("Unknow state '{}'", state)),
        }
    }
}

#[derive(Clone)]
pub struct Trigger {
    socket: ::std::cell::RefCell<Socket>,
}

impl ::Module for Trigger {
    fn get_socket<'a>(&'a self) -> ::std::cell::RefMut<'a, ::socket::Socket> {
        self.socket.borrow_mut()
    }
}

impl Trigger {
    pub fn new(socket: Socket) -> Self {
        Trigger {
            socket: ::std::cell::RefCell::new(socket),
        }
    }

    /**
     * Trigger immediately or set trigger source & edge.
     *
     * https://forum.redpitaya.com/viewtopic.php?f=14&t=1014
     */
    pub fn enable(&self, source: Source) {
        self.send(format!("ACQ:TRIG {}", Into::<String>::into(source)));
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
    pub fn get_state(&self) -> Result<State, String> {
        self.send("ACQ:TRIG:STAT?");

        self.receive()
            .parse()
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
    pub fn get_delay(&self) -> Result<u16, <u16 as ::std::str::FromStr>::Err> {
        self.send("ACQ:TRIG:DLY?");

        self.receive()
            .parse()
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
    pub fn get_delay_in_ns(&self) -> Result<u8, <u8 as ::std::str::FromStr>::Err> {
        self.send("ACQ:TRIG:DLY:NS?");

        self.receive()
            .replace("ns", "")
            .parse()
    }

    /**
     * Sets the trigger threshold hysteresis value in volts.
     *
     * Value must be outside to enable the trigger again.
     */
    pub fn set_hysteresis(&self, hysteresis: f32) {
        self.send(format!("ACQ:TRIG:HYST {}", hysteresis));
    }

    /**
     * Gets currently set trigger threshold hysteresis value in volts.
     */
    pub fn get_hysteresis(&self) -> Result<f32, <f32 as ::std::str::FromStr>::Err> {
        self.send("ACQ:TRIG:HYST?");

        self.receive()
            .parse()
    }

    /**
     * Set trigger level in mV.
     */
    pub fn set_level(&self, level: f32) {
        self.send(format!("ACQ:TRIG:LEV {}", level));
    }

    /**
     * Get trigger level in mV.
     */
    pub fn get_level(&self) -> Result<f32, <f32 as ::std::str::FromStr>::Err> {
        self.send("ACQ:TRIG:LEV?");

        self.receive()
            .replace("mV", "")
            .parse()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_status() {
        let (rx, trigger) = create_trigger();

        trigger.enable(::trigger::Source::NOW);
        assert_eq!("ACQ:TRIG NOW\r\n", rx.recv().unwrap());

        assert_eq!(trigger.get_state(), Ok(::trigger::State::WAIT));

        trigger.disable();
        assert_eq!("ACQ:TRIG DISABLED\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_delay() {
        let (rx, trigger) = create_trigger();

        trigger.set_delay(2314);
        assert_eq!("ACQ:TRIG:DLY 2314\r\n", rx.recv().unwrap());

        assert_eq!(trigger.get_delay(), Ok(2314));
    }

    #[test]
    fn test_delay_in_ns() {
        let (rx, trigger) = create_trigger();

        trigger.set_delay_in_ns(128);
        assert_eq!("ACQ:TRIG:DLY:NS 128\r\n", rx.recv().unwrap());

        assert_eq!(trigger.get_delay_in_ns(), Ok(128));
    }

    #[test]
    fn test_hysteresis() {
        let (rx, trigger) = create_trigger();

        trigger.set_hysteresis(0.75);
        assert_eq!("ACQ:TRIG:HYST 0.75\r\n", rx.recv().unwrap());

        assert_eq!(trigger.get_hysteresis(), Ok(0.75));
    }

    #[test]
    fn test_level() {
        let (rx, trigger) = create_trigger();

        trigger.set_level(123.0);
        assert_eq!("ACQ:TRIG:LEV 123\r\n", rx.recv().unwrap());

        assert_eq!(trigger.get_level(), Ok(123.0));
    }

    fn create_trigger() -> (::std::sync::mpsc::Receiver<String>, ::trigger::Trigger) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::trigger::Trigger::new(socket))
    }
}
