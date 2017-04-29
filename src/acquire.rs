use socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl ::std::convert::From<String> for Gain {
    fn from(s: String) -> Self {
        match s.as_str() {
            "LV" => Gain::LV,
            "HV" => Gain::HV,
            _ => Gain::LV,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Decimation {
    DEC_1,
    DEC_8,
    DEC_64,
    DEC_1024,
    DEC_8192,
    DEC_65536,
}

impl Decimation {
    pub fn get_buffer_duration(&self) -> ::std::time::Duration {
        let (s, ns) = match self {
            &Decimation::DEC_1 => (0, 131_072),
            &Decimation::DEC_8 => (0, 1_049_000),
            &Decimation::DEC_64 => (0, 8_389_000),
            &Decimation::DEC_1024 => (0, 134_218_000),
            &Decimation::DEC_8192 => (1, 740_000_000),
            &Decimation::DEC_65536 => (8, 590_000_000),
        };

        ::std::time::Duration::new(s, ns)
    }

    pub fn get_sampling_rate(&self) -> &'static str {
        match self {
            &Decimation::DEC_1 => "125 MS/s",
            &Decimation::DEC_8 => "15.6 MS/s",
            &Decimation::DEC_64 => "1.9 MS/s",
            &Decimation::DEC_1024 => "122.0 MS/s",
            &Decimation::DEC_8192 => "15.2 kS/s",
            &Decimation::DEC_65536 => "7.6 kS/s",
        }
    }
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SamplingRate {
    RATE_125MHz,
    RATE_15_6MHz,
    RATE_1_9MHz,
    RATE_103_8kHz,
    RATE_15_2kHz,
    RATE_1_9kHz,
}

impl ::std::fmt::Display for SamplingRate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &SamplingRate::RATE_125MHz => "125MHz",
            &SamplingRate::RATE_15_6MHz => "15_6MHz",
            &SamplingRate::RATE_1_9MHz => "1_9MHz",
            &SamplingRate::RATE_103_8kHz => "103_8kHz",
            &SamplingRate::RATE_15_2kHz => "15_2kHz",
            &SamplingRate::RATE_1_9kHz => "1_9kHz",
        };

        write!(f, "{}", display)
    }
}

impl ::std::convert::From<String> for SamplingRate {
    fn from(s: String) -> Self {
        match s.as_str() {
            "125MHz" => SamplingRate::RATE_125MHz,
            "15_6MHz" => SamplingRate::RATE_15_6MHz,
            "1_9MHz" => SamplingRate::RATE_1_9MHz,
            "103_8kHz" => SamplingRate::RATE_103_8kHz,
            "15_2kHz" => SamplingRate::RATE_15_2kHz,
            "1_9kHz" => SamplingRate::RATE_1_9kHz,
            _ => SamplingRate::RATE_125MHz,
        }
    }
}

#[derive(Clone)]
pub struct Acquire {
    socket: ::std::cell::RefCell<Socket>,
    started: bool,
}

impl Acquire {
    pub fn new(socket: Socket) -> Self {
        Acquire {
            socket: ::std::cell::RefCell::new(socket),
            started: false,
        }
    }

    /**
     * Starts acquisition.
     */
    pub fn start(&mut self) {
        self.send("ACQ:START");
        self.started = true;
    }

    /**
     * Stops acquisition.
     */
    pub fn stop(&mut self) {
        self.send("ACQ:STOP");
        self.started = false;
    }

    pub fn is_started(&self) -> bool {
        self.started
    }

    /**
     * Stops acquisition and sets all parameters to default values.
     */
    pub fn reset(&self) {
        self.send("ACQ:RST");
    }

    /**
     * Set decimation factor.
     */
    pub fn set_decimation(&self, decimation: Decimation) {
        self.send(format!("ACQ:DEC {}", decimation));
    }

    /**
     * Get decimation factor.
     */
    pub fn get_decimation(&self) -> Decimation {
        self.send("ACQ:DEC?");

        self.receive()
            .into()
    }

    /**
     * Set sampling rate.
     */
    pub fn set_sampling_rate(&self, rate: SamplingRate) {
        self.send(format!("ACQ:SRAT {}", rate));
    }

    /**
     * Get sampling rate.
     */
    pub fn get_sampling_rate(&self) -> SamplingRate {
        self.send("ACQ:SRAT?");

        self.receive()
            .into()
    }

    /**
     * Get sampling rate in Hertz.
     */
    pub fn get_sampling_rate_in_hertz(&self) -> u64 {
        self.send("ACQ:SRA:HZ?");

        self.receive()
            .replace(" Hz", "")
            .parse()
            .unwrap()
    }

    /**
     * Enable averaging.
     */
    pub fn enable_average(&self) {
        self.send("ACQ:AVG ON");
    }

    /**
     * Disable averaging.
     */
    pub fn disable_average(&self) {
        self.send("ACQ:AVG OFF");
    }

    /**
     * Get averaging status.
     */
    pub fn is_average_enabled(&self) -> bool {
        self.send("ACQ:AVG?");

        let message = self.receive();

        match message.as_str() {
            "ON" => true,
            _ => false,
        }
    }

    /**
     * Set gain settings to HIGH or LOW.
     *
     * This gain is referring to jumper settings on Red Pitaya fast analog inputs.
     */
    pub fn set_gain(&self, source: Source, gain: Gain) {
        self.send(format!("ACQ:{}:GAIN {}", source, gain));
    }

    /**
     * Get gain settings to HIGH or LOW.
     */
    pub fn get_gain(&self, source: Source) -> Gain {
        self.send(format!("ACQ:{}:GAIN?", source));

        self.receive()
            .into()
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
    #[test]
    fn test_decimation_get_buffer_duration() {
        let duration = ::std::time::Duration::new(8, 590_000_000);

        assert_eq!(duration, ::acquire::Decimation::DEC_65536.get_buffer_duration());
    }

    macro_rules! acquire_assert {
        ($f:ident, $e:expr) => {
            let (rx, acquire) = create_acquire();

            acquire.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    #[test]
    fn test_start() {
        let (rx, mut acquire) = create_acquire();

        acquire.start();
        assert_eq!("ACQ:START\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_stop() {
        let (rx, mut acquire) = create_acquire();

        acquire.stop();
        assert_eq!("ACQ:STOP\r\n", rx.recv().unwrap());
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
        let (rx, acquire) = create_acquire();

        acquire.set_decimation(::acquire::Decimation::DEC_8);
        assert_eq!("ACQ:DEC 8\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_decimation() {
        let (_, acquire) = create_acquire();

        assert_eq!(acquire.get_decimation(), ::acquire::Decimation::DEC_1);
    }

    #[test]
    fn test_set_sampling_rate() {
        let (rx, acquire) = create_acquire();

        acquire.set_sampling_rate(::acquire::SamplingRate::RATE_125MHz);
        assert_eq!("ACQ:SRAT 125MHz\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_sampling_rate() {
        let (_, acquire) = create_acquire();

        assert_eq!(acquire.get_sampling_rate(), ::acquire::SamplingRate::RATE_1_9kHz);
    }

    #[test]
    fn test_get_sampling_rate_in_hertz() {
        let (_, acquire) = create_acquire();

        assert_eq!(acquire.get_sampling_rate_in_hertz(), 125_000_000);
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
        let (_, acquire) = create_acquire();

        assert_eq!(acquire.is_average_enabled(), true);
    }

    #[test]
    fn test_set_gain() {
        let (rx, acquire) = create_acquire();

        acquire.set_gain(::acquire::Source::IN1, ::acquire::Gain::LV);
        assert_eq!("ACQ:SOUR1:GAIN LV\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_gain() {
        let (_, acquire) = create_acquire();

        assert_eq!(acquire.get_gain(::acquire::Source::IN1), ::acquire::Gain::HV);
    }

    fn create_acquire() -> (::std::sync::mpsc::Receiver<String>, ::acquire::Acquire) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::acquire::Acquire::new(socket))
    }
}
