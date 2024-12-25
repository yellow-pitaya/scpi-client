use crate::socket::Socket;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TriggerSource {
    EXT_PE,
    EXT_NE,
    INT,
    BURST,
}

impl std::convert::From<TriggerSource> for String {
    fn from(source: TriggerSource) -> Self {
        let s = match source {
            TriggerSource::EXT_PE => "EXT_PE",
            TriggerSource::EXT_NE => "EXT_NE",
            TriggerSource::INT => "INT",
            TriggerSource::BURST => "BURST",
        };

        String::from(s)
    }
}

impl std::str::FromStr for TriggerSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EXT_PE" => Ok(TriggerSource::EXT_PE),
            "EXT_NE" => Ok(TriggerSource::EXT_NE),
            "INT" => Ok(TriggerSource::INT),
            "BURST" => Ok(TriggerSource::BURST),
            source => Err(format!("Unknow source '{source}'")),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Form {
    SINE,
    SQUARE,
    TRIANGLE,
    SAWU,
    SAWD,
    DC,
    PWM,
    ARBITRARY,
}

impl std::convert::From<Form> for String {
    fn from(form: Form) -> Self {
        let s = match form {
            Form::SINE => "SINE",
            Form::SQUARE => "SQUARE",
            Form::TRIANGLE => "TRIANGLE",
            Form::SAWU => "SAWU",
            Form::SAWD => "SAWD",
            Form::DC => "DC",
            Form::PWM => "PWM",
            Form::ARBITRARY => "ARBITRARY",
        };

        String::from(s)
    }
}

impl std::str::FromStr for Form {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SINE" => Ok(Form::SINE),
            "SQUARE" => Ok(Form::SQUARE),
            "TRIANGLE" => Ok(Form::TRIANGLE),
            "SAWU" => Ok(Form::SAWU),
            "SAWD" => Ok(Form::SAWD),
            "DC" => Ok(Form::DC),
            "PWM" => Ok(Form::PWM),
            "ARBITRARY" => Ok(Form::ARBITRARY),
            form => Err(format!("Unknow signal form '{form}'")),
        }
    }
}

impl std::fmt::Display for Form {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Form::SINE => "Sine",
            Form::SQUARE => "Square",
            Form::TRIANGLE => "Triangle",
            Form::SAWU => "SAWU",
            Form::SAWD => "SAWD",
            Form::DC => "DC",
            Form::PWM => "PWM",
            Form::ARBITRARY => "Arbitrary",
        };

        write!(f, "{display}")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Source {
    OUT1,
    OUT2,
}

impl std::convert::From<Source> for String {
    fn from(source: Source) -> Self {
        let s = match source {
            Source::OUT1 => "SOUR1",
            Source::OUT2 => "SOUR2",
        };

        String::from(s)
    }
}

// @TODO impl std::slice::SliceIndex<generator::State> instead
impl std::convert::From<Source> for usize {
    fn from(source: Source) -> Self {
        match source {
            Source::OUT1 => 0,
            Source::OUT2 => 1,
        }
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Source::OUT1 => "OUT 1",
            Source::OUT2 => "OUT 2",
        };

        write!(f, "{display}")
    }
}

#[derive(Clone, Debug)]
pub struct Generator {
    socket: Socket,
}

impl crate::Module for Generator {
    fn new(socket: Socket) -> Self {
        Generator { socket }
    }
}

impl Generator {
    /**
     * Enable fast analog outputs.
     */
    pub fn start(&self, source: Source) {
        self.set_state(source, "ON");
    }

    /**
     * Disable fast analog outputs.
     */
    pub fn stop(&self, source: Source) {
        self.set_state(source, "OFF");
    }

    fn set_state(&self, source: Source, state: &str) {
        let output = match source {
            Source::OUT1 => "OUTPUT1",
            Source::OUT2 => "OUTPUT2",
        };

        self.socket.send(format!("{output}:STATE {state}"));
    }

    #[must_use]
    pub fn is_started(&self, source: Source) -> bool {
        let output = match source {
            Source::OUT1 => "OUTPUT1",
            Source::OUT2 => "OUTPUT2",
        };

        self.socket.send(format!("{output}:STATE?")) == Some("ON".to_owned())
    }

    /**
     * Set frequency of fast analog outputs.
     */
    pub fn set_frequency(&self, source: Source, frequency: u32) {
        self.socket.send(format!(
            "{}:FREQ:FIX {frequency}",
            Into::<String>::into(source),
        ));
    }

    /**
     * Get frequency of fast analog outputs.
     */
    pub fn frequency(&self, source: Source) -> Result<u32, <f32 as std::str::FromStr>::Err> {
        let value: f32 = self
            .socket
            .send(format!("{}:FREQ:FIX?", Into::<String>::into(source)))
            .unwrap()
            .parse()?;

        Ok(value as u32)
    }

    /**
     * Set waveform of fast analog outputs.
     *
     * PWM doesn’t work https://github.com/RedPitaya/RedPitaya/issues/81
     */
    pub fn set_form(&self, source: Source, form: Form) {
        self.socket.send(format!(
            "{}:FUNC {}",
            Into::<String>::into(source),
            Into::<String>::into(form)
        ));
    }

    pub fn form(&self, source: Source) -> Result<Form, String> {
        self.socket
            .send(format!("{}:FUNC?", Into::<String>::into(source)))
            .unwrap()
            .parse()
    }

    /**
     * Set amplitude voltage of fast analog outputs.
     *
     * Amplitude + offset value must be less than maximum output range ± 1V
     */
    pub fn set_amplitude(&self, source: Source, amplitude: f32) {
        self.socket.send(format!(
            "{}:VOLT {}",
            Into::<String>::into(source),
            amplitude
        ));
    }

    /**
     * Get amplitude voltage of fast analog outputs.
     */
    pub fn amplitude(&self, source: Source) -> Result<f32, <f32 as std::str::FromStr>::Err> {
        self.socket
            .send(format!("{}:VOLT?", Into::<String>::into(source)))
            .unwrap()
            .parse()
    }

    /**
     * Set offset voltage of fast analog outputs.
     *
     * Amplitude + offset value must be less than maximum output range ± 1V
     */
    pub fn set_offset(&self, source: Source, offset: f32) {
        self.socket.send(format!(
            "{}:VOLT:OFFS {}",
            Into::<String>::into(source),
            offset
        ));
    }

    /**
     * Get offset voltage of fast analog outputs.
     */
    pub fn offset(&self, source: Source) -> Result<f32, <f32 as std::str::FromStr>::Err> {
        self.socket
            .send(format!("{}:VOLT:OFFS?", Into::<String>::into(source)))
            .unwrap()
            .parse()
    }

    /**
     * Set phase of fast analog outputs.
     */
    pub fn set_phase(&self, source: Source, phase: i32) {
        self.socket
            .send(format!("{}:PHAS {}", Into::<String>::into(source), phase));
    }

    /**
     * Get phase of fast analog outputs.
     */
    pub fn phase(&self, source: Source) -> Result<i32, <i32 as std::str::FromStr>::Err> {
        self.socket
            .send(format!("{}:PHAS?", Into::<String>::into(source)))
            .unwrap()
            .parse()
    }

    /**
     * Set duty cycle of PWM waveform.
     */
    pub fn set_duty_cycle(&self, source: Source, dcyc: f32) {
        self.socket
            .send(format!("{}:DCYC {}", Into::<String>::into(source), dcyc));
    }

    /**
     * Get duty cycle of PWM waveform.
     */
    pub fn duty_cycle(&self, source: Source) -> Result<f32, <f32 as std::str::FromStr>::Err> {
        self.socket
            .send(format!("{}:DCYC?", Into::<String>::into(source)))
            .unwrap()
            .parse()
    }

    /**
     * Import data for arbitrary waveform generation.
     */
    pub fn set_arbitrary_waveform(&self, source: Source, data: &[f32]) {
        let mut data = data
            .iter()
            .fold(String::new(), |acc, e| format!("{acc}{e},"));
        data.pop();

        self.socket.send(format!(
            "{}:TRAC:DATA:DATA {data}",
            Into::<String>::into(source)
        ));
    }

    /**
     * Get data for arbitrary waveform generation.
     */
    #[must_use]
    pub fn arbitrary_waveform(&self, source: Source) -> Vec<f32> {
        let data = self
            .socket
            .send(format!("{}:TRAC:DATA:DATA?", Into::<String>::into(source)))
            .unwrap();

        data.trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect()
    }

    /**
     * Set trigger source for selected signal.
     */
    pub fn set_trigger_source(&self, source: Source, trigger: TriggerSource) {
        self.socket.send(format!(
            "{}:TRIG:SOUR {}",
            Into::<String>::into(source),
            Into::<String>::into(trigger)
        ));
    }

    /**
     * Get trigger source for selected signal.
     */
    pub fn trigger_source(&self, source: Source) -> Result<TriggerSource, String> {
        self.socket
            .send(format!("{}:TRIG:SOUR?", Into::<String>::into(source)))
            .unwrap()
            .parse()
    }

    /**
     * Triggers selected source immediately.
     */
    pub fn trigger(&self, source: Source) {
        self.socket
            .send(format!("{}:TRIG:IMM", Into::<String>::into(source)));
    }

    /**
     * Reset generator to default settings.
     */
    pub fn reset(&self) {
        self.socket.send("GEN:RST");
    }
}

#[cfg(test)]
mod test {
    macro_rules! generator_assert {
        ($f:ident, $e:expr) => {
            let (rx, rp) = crate::test::create_client();

            rp.generator.$f();
            assert_eq!($e, rx.recv().unwrap());
        };
    }

    #[test]
    fn test_status() {
        let (rx, rp) = crate::test::create_client();

        rp.generator.start(crate::generator::Source::OUT2);
        assert_eq!("OUTPUT2:STATE ON\r\n", rx.recv().unwrap());

        assert_eq!(
            rp.generator.is_started(crate::generator::Source::OUT2),
            true
        );

        rp.generator.stop(crate::generator::Source::OUT2);
        assert_eq!("OUTPUT2:STATE OFF\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_frequency() {
        let (rx, rp) = crate::test::create_client();

        rp.generator
            .set_frequency(crate::generator::Source::OUT1, 1_000);
        assert_eq!("SOUR1:FREQ:FIX 1000\r\n", rx.recv().unwrap());

        assert_eq!(
            rp.generator.frequency(crate::generator::Source::OUT1),
            Ok(1_000)
        );

        assert_eq!(
            rp.generator.frequency(crate::generator::Source::OUT1),
            Ok(1_000)
        );
    }

    #[test]
    fn test_form() {
        let (rx, rp) = crate::test::create_client();

        rp.generator
            .set_form(crate::generator::Source::OUT1, crate::generator::Form::SINE);
        assert_eq!("SOUR1:FUNC SINE\r\n", rx.recv().unwrap());

        assert_eq!(
            rp.generator.form(crate::generator::Source::OUT1),
            Ok(crate::generator::Form::SINE)
        );
    }

    #[test]
    fn test_amplitude() {
        let (rx, rp) = crate::test::create_client();

        rp.generator.start(crate::generator::Source::OUT1);
        assert_eq!("OUTPUT1:STATE ON\r\n", rx.recv().unwrap());

        rp.generator
            .set_amplitude(crate::generator::Source::OUT1, -0.5);
        assert_eq!("SOUR1:VOLT -0.5\r\n", rx.recv().unwrap());

        assert_eq!(
            rp.generator.amplitude(crate::generator::Source::OUT1),
            Ok(-0.5)
        );
    }

    #[test]
    fn test_offset() {
        let (rx, rp) = crate::test::create_client();

        rp.generator.set_offset(crate::generator::Source::OUT1, 0.3);
        assert_eq!("SOUR1:VOLT:OFFS 0.3\r\n", rx.recv().unwrap());

        assert_eq!(rp.generator.offset(crate::generator::Source::OUT1), Ok(0.3));
    }

    #[test]
    fn test_phase() {
        let (rx, rp) = crate::test::create_client();

        rp.generator.set_phase(crate::generator::Source::OUT1, 180);
        assert_eq!("SOUR1:PHAS 180\r\n", rx.recv().unwrap());

        assert_eq!(rp.generator.phase(crate::generator::Source::OUT1), Ok(180));
    }

    #[test]
    fn test_duty_cycle() {
        let (rx, rp) = crate::test::create_client();

        rp.generator
            .set_duty_cycle(crate::generator::Source::OUT1, 1.0);
        assert_eq!("SOUR1:DCYC 1\r\n", rx.recv().unwrap());

        assert_eq!(
            rp.generator.duty_cycle(crate::generator::Source::OUT1),
            Ok(1.0)
        );
    }

    #[test]
    fn test_arbitrary_waveform() {
        let (rx, rp) = crate::test::create_client();

        rp.generator
            .set_arbitrary_waveform(crate::generator::Source::OUT1, &[1.0, 0.5, 0.2]);
        assert_eq!("SOUR1:TRAC:DATA:DATA 1,0.5,0.2\r\n", rx.recv().unwrap());

        #[cfg(feature = "mock")]
        assert_eq!(
            rp.generator
                .arbitrary_waveform(crate::generator::Source::OUT1),
            vec![1.0, 0.5, 0.2]
        );

        #[cfg(not(feature = "mock"))]
        assert!(
            rp.generator
                .arbitrary_waveform(crate::generator::Source::OUT1)
                .len()
                > 0
        );
    }

    #[test]
    fn test_trigger_source() {
        let (rx, rp) = crate::test::create_client();

        rp.generator.set_trigger_source(
            crate::generator::Source::OUT1,
            crate::generator::TriggerSource::BURST,
        );
        assert_eq!("SOUR1:TRIG:SOUR BURST\r\n", rx.recv().unwrap());

        assert_eq!(
            rp.generator.trigger_source(crate::generator::Source::OUT1),
            Ok(crate::generator::TriggerSource::BURST)
        );

        rp.generator.set_trigger_source(
            crate::generator::Source::OUT1,
            crate::generator::TriggerSource::INT,
        );
        assert_eq!("SOUR1:TRIG:SOUR INT\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_trigger() {
        let (rx, rp) = crate::test::create_client();

        rp.generator.trigger(crate::generator::Source::OUT1);
        assert_eq!("SOUR1:TRIG:IMM\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_reset() {
        generator_assert!(reset, "GEN:RST\r\n");
    }
}
