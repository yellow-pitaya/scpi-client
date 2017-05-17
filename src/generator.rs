use Module;
use socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TriggerSource {
    EXT_PE,
    EXT_NE,
    INT,
    GATED,
}

impl ::std::convert::Into<String> for TriggerSource {
    fn into(self) -> String {
        let s = match self {
            TriggerSource::EXT_PE => "EXT_PE",
            TriggerSource::EXT_NE => "EXT_NE",
            TriggerSource::INT => "INT",
            TriggerSource::GATED => "GATED",
        };

        String::from(s)
    }
}

impl ::std::str::FromStr for TriggerSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EXT_PE" => Ok(TriggerSource::EXT_PE),
            "EXT_NE" => Ok(TriggerSource::EXT_NE),
            "INT" => Ok(TriggerSource::INT),
            "GATED" => Ok(TriggerSource::GATED),
            source => Err(format!("Unknow source '{}'", source)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Form {
    SINE,
    SQUARE,
    TRIANGLE,
    SAWU,
    SAWD,
    PWM,
    ARBITRARY,
}

impl ::std::convert::Into<String> for Form {
    fn into(self) -> String {
        let s = match self {
            Form::SINE => "SINE",
            Form::SQUARE => "SQUARE",
            Form::TRIANGLE => "TRIANGLE",
            Form::SAWU => "SAWU",
            Form::SAWD => "SAWD",
            Form::PWM => "PWM",
            Form::ARBITRARY => "ARBITRARY",
        };

        String::from(s)
    }
}

impl ::std::str::FromStr for Form {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SINE" => Ok(Form::SINE),
            "SQUARE" => Ok(Form::SQUARE),
            "TRIANGLE" => Ok(Form::TRIANGLE),
            "SAWU" => Ok(Form::SAWU),
            "SAWD" => Ok(Form::SAWD),
            "PWM" => Ok(Form::PWM),
            "ARBITRARY" => Ok(Form::ARBITRARY),
            form => Err(format!("Unknow signal form '{}'", form)),
        }
    }
}

impl ::std::fmt::Display for Form {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Form::SINE => "Sine",
            &Form::SQUARE => "Square",
            &Form::TRIANGLE => "Triangle",
            &Form::SAWU => "SAWU",
            &Form::SAWD => "SAWD",
            &Form::PWM => "PWM",
            &Form::ARBITRARY => "Arbitrary",
        };

        write!(f, "{}", display)
    }
}

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

// @TODO impl std::slice::SliceIndex<generator::State> instead
impl ::std::convert::Into<usize> for Source {
    fn into(self) -> usize {
        match self {
            Source::OUT1 => 0,
            Source::OUT2 => 1,
        }
    }
}

impl ::std::fmt::Display for Source {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Source::OUT1 => "OUT 1",
            &Source::OUT2 => "OUT 2",
        };

        write!(f, "{}", display)
    }
}

#[derive(Clone)]
pub struct Generator {
    socket: ::std::cell::RefCell<Socket>,
}

impl ::Module for Generator {
    fn get_socket<'a>(&'a self) -> ::std::cell::RefMut<'a, ::socket::Socket> {
        self.socket.borrow_mut()
    }
}

impl Generator {
    pub fn new(socket: ::std::cell::RefCell<Socket>) -> Self {
        Generator {
            socket,
        }
    }

    /**
     * Enable fast analog outputs.
     */
    pub fn start(&self, source: Source) {
        self.set_state(&source, "ON");
    }

    /**
     * Disable fast analog outputs.
     */
    pub fn stop(&self, source: Source) {
        self.set_state(&source, "OFF");
    }

    fn set_state(&self, source: &Source, state: &str) {
        let output = match *source {
            Source::OUT1 => "OUTPUT1",
            Source::OUT2 => "OUTPUT2",
        };

        self.send(format!("{}:STATE {}", output, state));
    }

    pub fn is_started(&self, source: Source) -> Result<bool, <i8 as ::std::str::FromStr>::Err> {
        let output = match source {
            Source::OUT1 => "OUTPUT1",
            Source::OUT2 => "OUTPUT2",
        };

        self.send(format!("{}:STATE?", output));

        Ok(self.receive() == "ON")
    }

    /**
     * Set frequency of fast analog outputs.
     */
    pub fn set_frequency(&self, source: Source, frequency: u32) {
        self.send(format!("{}:FREQ:FIX {}", Into::<String>::into(source), frequency));
    }

    /**
     * Get frequency of fast analog outputs.
     */
    pub fn get_frequency(&self, source: Source) -> Result<u32, <f32 as ::std::str::FromStr>::Err> {
        self.send(format!("{}:FREQ:FIX?", Into::<String>::into(source)));

        let value: f32 = self.receive()
            .parse()?;

        Ok(value as u32)
    }

    /**
     * Set waveform of fast analog outputs.
     *
     * PWM doesn’t work https://github.com/RedPitaya/RedPitaya/issues/81
     */
    pub fn set_form(&self, source: Source, form: Form) {
        self.send(format!("{}:FUNC {}", Into::<String>::into(source), Into::<String>::into(form)));
    }

    pub fn get_form(&self, source: Source) -> Result<Form, String> {
        self.send(format!("{}:FUNC?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }

    /**
     * Set amplitude voltage of fast analog outputs.
     *
     * Amplitude + offset value must be less than maximum output range ± 1V
     */
    pub fn set_amplitude(&self, source: Source, amplitude: f32) {
        self.send(format!("{}:VOLT {}", Into::<String>::into(source), amplitude));
    }

    /**
     * Get amplitude voltage of fast analog outputs.
     */
    pub fn get_amplitude(&self, source: Source) -> Result<f32, <f32 as ::std::str::FromStr>::Err> {
        self.send(format!("{}:VOLT?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }

    /**
     * Set offset voltage of fast analog outputs.
     *
     * Amplitude + offset value must be less than maximum output range ± 1V
     */
    pub fn set_offset(&self, source: Source, offset: f32) {
        self.send(format!("{}:VOLT:OFFS {}", Into::<String>::into(source), offset));
    }

    /**
     * Get offset voltage of fast analog outputs.
     */
    pub fn get_offset(&self, source: Source) -> Result<f32, <f32 as ::std::str::FromStr>::Err> {
        self.send(format!("{}:VOLT:OFFS?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }

    /**
     * Set phase of fast analog outputs.
     */
    pub fn set_phase(&self, source: Source, phase: i32) {
        self.send(format!("{}:PHAS {}", Into::<String>::into(source), phase));
    }

    /**
     * Get phase of fast analog outputs.
     */
    pub fn get_phase(&self, source: Source) -> Result<i32, <i32 as ::std::str::FromStr>::Err> {
        self.send(format!("{}:PHAS?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }

    /**
     * Set duty cycle of PWM waveform.
     */
    pub fn set_duty_cycle(&self, source: Source, dcyc: f32) {
        self.send(format!("{}:DCYC {}", Into::<String>::into(source), dcyc));
    }

    /**
     * Get duty cycle of PWM waveform.
     */
    pub fn get_duty_cycle(&self, source: Source) -> Result<f32, <f32 as ::std::str::FromStr>::Err> {
        self.send(format!("{}:DCYC?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }

    /**
     * Import data for arbitrary waveform generation.
     */
    pub fn set_arbitrary_waveform(&self, source: Source, data: Vec<f32>) {
        let mut data = data.iter()
            .fold(String::new(), |acc, e| {
                format!("{}{},", acc, e)
            });
        data.pop();

        self.send(format!("{}:TRAC:DATA:DATA {}", Into::<String>::into(source), data));
    }

    /**
     * Get data for arbitrary waveform generation.
     */
    pub fn get_arbitrary_waveform(&self, source: Source) -> Vec<f32> {
        self.send(format!("{}:TRAC:DATA:DATA?", Into::<String>::into(source)));

        let data = self.receive();
        data.trim_matches(|c| c == '{' || c == '}')
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect()
    }

    /**
     * Set trigger source for selected signal.
     */
    pub fn set_trigger_source(&self, source: Source, trigger: TriggerSource) {
        self.send(format!("{}:TRIG:SOUR {}", Into::<String>::into(source), Into::<String>::into(trigger)));
    }

    /**
     * Get trigger source for selected signal.
     */
    pub fn get_trigger_source(&self, source: Source) -> Result<TriggerSource, String> {
        self.send(format!("{}:TRIG:SOUR?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }

    /**
     * Triggers selected source immediately.
     */
    pub fn trigger(&self, source: Source) {
        self.send(format!("{}:TRIG:IMM", Into::<String>::into(source)));
    }

    /**
     * Triggers both sources immediately.
     */
    pub fn trigger_all(&self) {
        self.send("TRIG:IMM");
    }

    /**
     * Reset generator to default settings.
     */
    pub fn reset(&self) {
        self.send("GEN:RST");
    }
}

#[cfg(test)]
mod test {
    macro_rules! generator_assert {
        ($f:ident, $e:expr) => {
            let (rx, rp) = ::test::create_client();

            rp.generator.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    #[test]
    fn test_status() {
        let (rx, rp) = ::test::create_client();

        rp.generator.start(::generator::Source::OUT2);
        assert_eq!("OUTPUT2:STATE ON\r\n", rx.recv().unwrap());

        assert_eq!(rp.generator.is_started(::generator::Source::OUT2), Ok(true));

        rp.generator.stop(::generator::Source::OUT2);
        assert_eq!("OUTPUT2:STATE OFF\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_frequency() {
        let (rx, rp) = ::test::create_client();

        rp.generator.set_frequency(::generator::Source::OUT1, 1_000);
        assert_eq!("SOUR1:FREQ:FIX 1000\r\n", rx.recv().unwrap());

        assert_eq!(rp.generator.get_frequency(::generator::Source::OUT1), Ok(1_000));

        assert_eq!(rp.generator.get_frequency(::generator::Source::OUT1), Ok(1_000));
    }

    #[test]
    fn test_form() {
        let (rx, rp) = ::test::create_client();

        rp.generator.set_form(::generator::Source::OUT1, ::generator::Form::SINE);
        assert_eq!("SOUR1:FUNC SINE\r\n", rx.recv().unwrap());

        assert_eq!(rp.generator.get_form(::generator::Source::OUT1), Ok(::generator::Form::SINE));
    }

    #[test]
    fn test_amplitude() {
        let (rx, rp) = ::test::create_client();

        rp.generator.start(::generator::Source::OUT1);
        assert_eq!("OUTPUT1:STATE ON\r\n", rx.recv().unwrap());

        rp.generator.set_amplitude(::generator::Source::OUT1, -1.1);
        assert_eq!("SOUR1:VOLT -1.1\r\n", rx.recv().unwrap());

        assert_eq!(rp.generator.get_amplitude(::generator::Source::OUT1), Ok(-1.1));
    }

    #[test]
    fn test_offset() {
        let (rx, rp) = ::test::create_client();

        rp.generator.set_offset(::generator::Source::OUT1, -1.2);
        assert_eq!("SOUR1:VOLT:OFFS -1.2\r\n", rx.recv().unwrap());

        assert_eq!(rp.generator.get_offset(::generator::Source::OUT1), Ok(1.2));
    }

    #[test]
    fn test_phase() {
        let (rx, rp) = ::test::create_client();

        rp.generator.set_phase(::generator::Source::OUT1, 180);
        assert_eq!("SOUR1:PHAS 180\r\n", rx.recv().unwrap());

        assert_eq!(rp.generator.get_phase(::generator::Source::OUT1), Ok(180));
    }

    #[test]
    fn test_duty_cycle() {
        let (rx, rp) = ::test::create_client();

        rp.generator.set_duty_cycle(::generator::Source::OUT1, 1.0);
        assert_eq!("SOUR1:DCYC 1\r\n", rx.recv().unwrap());

        assert_eq!(rp.generator.get_duty_cycle(::generator::Source::OUT1), Ok(1.0));
    }

    #[test]
    fn test_arbitrary_waveform() {
        let (rx, rp) = ::test::create_client();

        rp.generator.set_arbitrary_waveform(::generator::Source::OUT1, vec![1.0, 0.5, 0.2]);
        assert_eq!("SOUR1:TRAC:DATA:DATA 1,0.5,0.2\r\n", rx.recv().unwrap());

        #[cfg(feature = "mock")]
        assert_eq!(rp.generator.get_arbitrary_waveform(::generator::Source::OUT1), vec![1.0, 0.5, 0.2]);

        #[cfg(not(feature = "mock"))]
        assert!(rp.generator.get_arbitrary_waveform(::generator::Source::OUT1).len() > 0);
    }

    #[test]
    fn test_trigger_source() {
        let (rx, rp) = ::test::create_client();

        rp.generator.set_trigger_source(::generator::Source::OUT1, ::generator::TriggerSource::EXT_NE);
        assert_eq!("SOUR1:TRIG:SOUR EXT_NE\r\n", rx.recv().unwrap());

        assert_eq!(rp.generator.get_trigger_source(::generator::Source::OUT1), Ok(::generator::TriggerSource::EXT_NE));
    }

    #[test]
    fn test_trigger() {
        let (rx, rp) = ::test::create_client();

        rp.generator.trigger(::generator::Source::OUT1);
        assert_eq!("SOUR1:TRIG:IMM\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_trigger_all() {
        generator_assert!(trigger_all, "TRIG:IMM\r\n");
    }

    #[test]
    fn test_reset() {
        generator_assert!(reset, "GEN:RST\r\n");
    }
}
