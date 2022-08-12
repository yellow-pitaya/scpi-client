use crate::socket::Socket;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::convert::From<Source> for String {
    fn from(source: Source) -> Self {
        let s = match source {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    WAIT,
    TD,
}

impl std::str::FromStr for State {
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
    socket: Socket,
}

impl crate::Module for Trigger {
    fn new(socket: Socket) -> Self {
        Trigger { socket }
    }
}

impl Trigger {
    /**
     * Trigger immediately or set trigger source & edge.
     *
     * https://forum.redpitaya.com/viewtopic.php?f=14&t=1014
     */
    pub fn enable(&self, source: Source) {
        self.socket
            .send(format!("ACQ:TRIG {}", Into::<String>::into(source)));
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
        self.socket.send("ACQ:TRIG:STAT?").unwrap().parse()
    }

    /**
     * Set trigger delay in samples.
     */
    pub fn set_delay(&self, delay: u16) {
        self.socket.send(format!("ACQ:TRIG:DLY {}", delay));
    }

    /**
     * Get trigger delay in samples.
     */
    pub fn get_delay(&self) -> Result<u16, <u16 as std::str::FromStr>::Err> {
        self.socket.send("ACQ:TRIG:DLY?").unwrap().parse()
    }

    /**
     * Set trigger delay in ns.
     */
    pub fn set_delay_in_ns(&self, delay: u8) {
        self.socket.send(format!("ACQ:TRIG:DLY:NS {}", delay));
    }

    /**
     * Get trigger delay in ns.
     */
    pub fn get_delay_in_ns(&self) -> Result<u8, <u8 as std::str::FromStr>::Err> {
        self.socket
            .send("ACQ:TRIG:DLY:NS?")
            .unwrap()
            .replace("ns", "")
            .parse()
    }

    /**
     * Sets the trigger threshold hysteresis value in volts.
     *
     * Value must be outside to enable the trigger again.
     */
    pub fn set_hysteresis(&self, hysteresis: f32) {
        self.socket.send(format!("ACQ:TRIG:HYST {}", hysteresis));
    }

    /**
     * Gets currently set trigger threshold hysteresis value in volts.
     */
    pub fn get_hysteresis(&self) -> Result<f32, <f32 as std::str::FromStr>::Err> {
        self.socket.send("ACQ:TRIG:HYST?").unwrap().parse()
    }

    /**
     * Set trigger level in mV.
     */
    pub fn set_level(&self, level: f32) {
        self.socket.send(format!("ACQ:TRIG:LEV {}", level));
    }

    /**
     * Get trigger level in mV.
     */
    pub fn get_level(&self) -> Result<f32, <f32 as std::str::FromStr>::Err> {
        self.socket
            .send("ACQ:TRIG:LEV?")
            .unwrap()
            .replace("mV", "")
            .parse()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_status() {
        let (rx, rp) = crate::test::create_client();

        rp.trigger.enable(crate::trigger::Source::NOW);
        assert_eq!("ACQ:TRIG NOW\r\n", rx.recv().unwrap());

        #[cfg(feature = "mock")]
        assert_eq!(rp.trigger.get_state(), Ok(crate::trigger::State::WAIT));

        #[cfg(not(feature = "mock"))]
        assert!(rp.trigger.get_state().is_ok());

        rp.trigger.disable();
        assert_eq!("ACQ:TRIG DISABLED\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_delay() {
        let (rx, rp) = crate::test::create_client();

        rp.trigger.set_delay(2314);
        assert_eq!("ACQ:TRIG:DLY 2314\r\n", rx.recv().unwrap());

        assert_eq!(rp.trigger.get_delay(), Ok(2314));
    }

    #[test]
    fn test_delay_in_ns() {
        let (rx, rp) = crate::test::create_client();

        rp.trigger.set_delay_in_ns(128);
        assert_eq!("ACQ:TRIG:DLY:NS 128\r\n", rx.recv().unwrap());

        assert_eq!(rp.trigger.get_delay_in_ns(), Ok(128));
    }

    #[test]
    fn test_hysteresis() {
        let (rx, rp) = crate::test::create_client();

        rp.trigger.set_hysteresis(0.75);
        assert_eq!("ACQ:TRIG:HYST 0.75\r\n", rx.recv().unwrap());

        assert_eq!(rp.trigger.get_hysteresis(), Ok(0.75));
    }

    #[test]
    fn test_level() {
        let (rx, rp) = crate::test::create_client();

        rp.trigger.set_level(0.4);
        assert_eq!("ACQ:TRIG:LEV 0.4\r\n", rx.recv().unwrap());

        assert_eq!(rp.trigger.get_level(), Ok(0.4));
    }
}
