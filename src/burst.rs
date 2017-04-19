use socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Source {
    OUT1,
    OUT2,
}

impl ::std::fmt::Display for Source {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Source::OUT1 => "SOUR1",
            &Source::OUT2 => "SOUR2",
        };

        write!(f, "{}", display)
    }
}

enum State {
    ON,
    OFF,
}

impl ::std::fmt::Display for State {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &State::ON => "ON",
            &State::OFF => "OFF",
        };

        write!(f, "{}", display)
    }
}

#[derive(Clone)]
pub struct Burst {
    socket: ::std::cell::RefCell<Socket>,
}

impl Burst {
    pub fn new(socket: Socket) -> Self {
        Burst {
            socket: ::std::cell::RefCell::new(socket),
        }
    }

    /**
     * Enable burst (pulse) mode.
     *
     * Red Pitaya will generate R number of N periods of signal and then stop. Time between bursts
     * is P.
     */
    pub fn enable(&self, source: Source) {
        self.send(format!("{}:BURS:STAT {}", source, State::ON));
    }

    /**
     * Disable burst mode.
     */
    pub fn disable(&self, source: Source) {
        self.send(format!("{}:BURS:STAT {}", source, State::OFF));
    }

    /**
     * Set N number of periods in one burst.
     */
    pub fn set_count(&self, source: Source, count: u32) {
        self.send(format!("{}:BURS:NCYC {}", source, count));
    }

    /**
     * Set R number of repeated bursts.
     */
    pub fn set_repetitions(&self, source: Source, repetitions: u32) {
        self.send(format!("{}:BURS:NOR {}", source, repetitions));
    }

    /**
     * Set P total time of one burst in in micro seconds.
     * This includes the signal and delay.
     */
    pub fn set_period(&self, source: Source, period: u32) {
        self.send(format!("{}:BURS:INT:PER {}", source, period));
    }

    fn send<D>(&self, message: D)
        where D: ::std::fmt::Display
    {
        let mut socket = self.socket.borrow_mut();

        socket.send(message);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_enable() {
        let (rx, burst) = create_burst();

        burst.enable(::burst::Source::OUT1);
        assert_eq!("SOUR1:BURS:STAT ON\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_disable() {
        let (rx, burst) = create_burst();

        burst.disable(::burst::Source::OUT1);
        assert_eq!("SOUR1:BURS:STAT OFF\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_count() {
        let (rx, burst) = create_burst();

        burst.set_count(::burst::Source::OUT1, 3);
        assert_eq!("SOUR1:BURS:NCYC 3\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_repetitions() {
        let (rx, burst) = create_burst();

        burst.set_repetitions(::burst::Source::OUT1, 5);
        assert_eq!("SOUR1:BURS:NOR 5\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_period() {
        let (rx, burst) = create_burst();

        burst.set_period(::burst::Source::OUT2, 1_000_000);
        assert_eq!("SOUR2:BURS:INT:PER 1000000\r\n", rx.recv().unwrap());
    }

    fn create_burst() -> (::std::sync::mpsc::Receiver<String>, ::burst::Burst) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::burst::Burst::new(socket))
    }
}
