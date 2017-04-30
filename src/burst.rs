use Module;
use socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Source {
    OUT1,
    OUT2,
}

impl ::std::convert::Into<String> for Source {
    fn into(self) -> String {
        let s = match self {
            Source::OUT1 => "SOUR1",
            Source::OUT2 => "SOUR2",
        };

        String::from(s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    ON,
    OFF,
}

impl ::std::convert::Into<String> for State {
    fn into(self) -> String {
        let s = match self {
            State::ON => "ON",
            State::OFF => "OFF",
        };

        String::from(s)
    }
}

impl ::std::convert::From<String> for State {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ON" => State::ON,
            "OFF" => State::OFF,
            _ => State::OFF,
        }
    }
}

#[derive(Clone)]
pub struct Burst {
    socket: ::std::cell::RefCell<Socket>,
}

impl ::Module for Burst {
    fn get_socket<'a>(&'a self) -> ::std::cell::RefMut<'a, ::socket::Socket> {
        self.socket.borrow_mut()
    }
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
        self.set_status(source, State::ON);
    }

    /**
     * Disable burst mode.
     */
    pub fn disable(&self, source: Source) {
        self.set_status(source, State::OFF);
    }

    fn set_status(&self, source: Source, state: State) {
        self.send(format!("{}:BURS:STAT {}", Into::<String>::into(source), Into::<String>::into(state)));
    }

    /**
     * Disable burst mode.
     */
    pub fn get_status(&self, source: Source) -> State {
        self.send(format!("{}:BURS:STAT?", Into::<String>::into(source)));

        self.receive()
            .into()
    }

    /**
     * Set N number of periods in one burst.
     */
    pub fn set_count(&self, source: Source, count: u32) {
        self.send(format!("{}:BURS:NCYC {}", Into::<String>::into(source), count));
    }

    /**
     * Get number of periods in one burst.
     */
    pub fn get_count(&self, source: Source) -> Result<u32, <u32 as ::std::str::FromStr>::Err> {
        self.send(format!("{}:BURS:NCYC?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }

    /**
     * Set R number of repeated bursts.
     */
    pub fn set_repetitions(&self, source: Source, repetitions: u32) {
        self.send(format!("{}:BURS:NOR {}", Into::<String>::into(source), repetitions));
    }

    /**
     * Get number of repeated bursts.
     */
    pub fn get_repetitions(&self, source: Source) -> Result<u32, <u32 as ::std::str::FromStr>::Err> {
        self.send(format!("{}:BURS:NOR?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }

    /**
     * Set P total time of one burst in in micro seconds.
     *
     * This includes the signal and delay.
     */
    pub fn set_period(&self, source: Source, period: u32) {
        self.send(format!("{}:BURS:INT:PER {}", Into::<String>::into(source), period));
    }

    /**
     * Get total time of one burst in in micro seconds.
     *
     * This includes the signal and delay.
     */
    pub fn get_period(&self, source: Source) -> Result<u32, <u32 as ::std::str::FromStr>::Err> {
        self.send(format!("{}:BURS:INT:PER?", Into::<String>::into(source)));

        self.receive()
            .parse()
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
    fn test_get_status() {
        let (_, burst) = create_burst();

        assert_eq!(burst.get_status(::burst::Source::OUT2), ::burst::State::OFF);
    }

    #[test]
    fn test_set_count() {
        let (rx, burst) = create_burst();

        burst.set_count(::burst::Source::OUT1, 3);
        assert_eq!("SOUR1:BURS:NCYC 3\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_count() {
        let (_, burst) = create_burst();

        assert_eq!(burst.get_count(::burst::Source::OUT2), Ok(3));
    }

    #[test]
    fn test_set_repetitions() {
        let (rx, burst) = create_burst();

        burst.set_repetitions(::burst::Source::OUT1, 5);
        assert_eq!("SOUR1:BURS:NOR 5\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_repetitions() {
        let (_, burst) = create_burst();

        assert_eq!(burst.get_repetitions(::burst::Source::OUT1), Ok(5));
    }

    #[test]
    fn test_set_period() {
        let (rx, burst) = create_burst();

        burst.set_period(::burst::Source::OUT2, 1_000_000);
        assert_eq!("SOUR2:BURS:INT:PER 1000000\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_period() {
        let (_, burst) = create_burst();

        assert_eq!(burst.get_period(::burst::Source::OUT2), Ok(1_000_000));
    }

    fn create_burst() -> (::std::sync::mpsc::Receiver<String>, ::burst::Burst) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::burst::Burst::new(socket))
    }
}
