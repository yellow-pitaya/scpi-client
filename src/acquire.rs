use socket::Socket;

pub enum Gain {
    LV,
    HV,
}

impl ::std::fmt::Display for Gain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Gain::LV => "LV",
            &Gain::HV => "HV",
        };

        write!(f, "{}", display)
    }
}

pub enum Source {
    IN1,
    IN2,
}

impl ::std::fmt::Display for Source {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Source::IN1 => "SOUR1",
            &Source::IN2 => "SOUR2",
        };

        write!(f, "{}", display)
    }
}

#[derive(Debug, PartialEq)]
pub enum Decimation {
    DEC_1,
    DEC_8,
    DEC_64,
    DEC_1024,
    DEC_8192,
    DEC_65536,
}

impl ::std::fmt::Display for Decimation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Decimation::DEC_1 => "1",
            &Decimation::DEC_8 => "8",
            &Decimation::DEC_64 => "64",
            &Decimation::DEC_1024 => "1024",
            &Decimation::DEC_8192 => "8192",
            &Decimation::DEC_65536 => "65536",
        };

        write!(f, "{}", display)
    }
}

impl ::std::convert::From<String> for Decimation {
    fn from(s: String) -> Self {
        match s.as_str() {
            "1" => Decimation::DEC_1,
            "8" => Decimation::DEC_8,
            "64" => Decimation::DEC_64,
            "1024" => Decimation::DEC_1024,
            "8192" => Decimation::DEC_8192,
            "65536" => Decimation::DEC_65536,
            _ => Decimation::DEC_1,
        }
    }
}

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

    pub fn set_decimation(&mut self, decimation: Decimation) {
        self.socket.send(format!("ACQ:DEC {}", decimation));
    }

    pub fn get_decimation(&mut self) -> Decimation {
        self.socket.send("ACQ:DEC?");

        self.socket.receive()
            .into()
    }

    pub fn enable_average(&mut self) {
        self.socket.send("ACQ:AVG ON");
    }

    pub fn disable_average(&mut self) {
        self.socket.send("ACQ:AVG OFF");
    }

    pub fn is_average_enabled(&mut self) -> bool {
        self.socket.send("ACQ:AVG?");

        let message = self.socket.receive();

        match message.as_str() {
            "ON" => true,
            _ => false,
        }
    }

    pub fn set_gain(&mut self, source: Source, gain: Gain) {
        self.socket.send(format!("ACQ:{}:GAIN {}", source, gain));
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
    fn test_set_decimation() {
        let (rx, mut acquire) = create_acquire();

        acquire.set_decimation(::acquire::Decimation::DEC_8);
        assert_eq!("ACQ:DEC 8\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_decimation() {
        let (_, mut acquire) = create_acquire();

        assert_eq!(acquire.get_decimation(), ::acquire::Decimation::DEC_1);
    }

    #[test]
    fn test_enable_average() {
        acquire_assert!(enable_average, "ACQ:AVG ON\r\n");
    }

    #[test]
    fn test_disable_average() {
        acquire_assert!(disable_average, "ACQ:AVG OFF\r\n");
    }

    #[test]
    fn test_is_average_enabled() {
        let (_, mut acquire) = create_acquire();

        assert_eq!(acquire.is_average_enabled(), true);
    }

    #[test]
    fn test_set_gain() {
        let (rx, mut trigger) = create_acquire();

        trigger.set_gain(::acquire::Source::IN1, ::acquire::Gain::LV);
        assert_eq!("ACQ:SOUR1:GAIN LV\r\n", rx.recv().unwrap());
    }

    fn create_acquire() -> (::std::sync::mpsc::Receiver<String>, ::acquire::Acquire) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(
            format!("{}", addr.ip()).as_str(),
            addr.port()
        );

        (rx, ::acquire::Acquire::new(socket))
    }
}
