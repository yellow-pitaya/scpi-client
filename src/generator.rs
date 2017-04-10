use socket::Socket;

pub struct Generator {
    socket: Socket,
    started: bool,
    form: String,
    amplitude: f32,
    frequency: u32,
    dcyc: u32,
}

impl Generator {
    pub fn new(socket: Socket) -> Self {
        Generator {
            socket: socket,
            started: false,
            form: "SYN".into(),
            amplitude: 1.0,
            frequency: 1000,
            dcyc: 50,
        }
    }

    pub fn start(&mut self) {
        self.socket.send("OUTPUT1:STATE ON");
        self.started = true;
    }

    pub fn stop(&mut self) {
        self.socket.send("OUTPUT1:STATE OFF");
        self.started = false;
    }

    pub fn is_started(&self) -> bool {
        self.started
    }

    pub fn set_form<S>(&mut self, form: S) where S: Into<String> {
        self.form = form.into();
        self.socket.send(format!("SOUR1:FUNC {}", self.form))
    }

    pub fn get_form(&self) -> String {
        self.form.clone()
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.socket.send(format!("SOUR1:VOLT {}", amplitude));
        self.amplitude = amplitude;
    }

    pub fn get_amplitude(&self) -> f32 {
        self.amplitude
    }

    pub fn set_frequency(&mut self, frequency: u32) {
        self.socket.send(format!("SOUR1:FREQ:FIX {}", frequency));
        self.frequency = frequency;
    }

    pub fn get_frequency(&self) -> u32 {
        self.frequency
    }

    pub fn set_dcyc(&mut self, dcyc: u32) {
        self.socket.send(format!("SOUR1:DCYC {}", dcyc));
        self.dcyc = dcyc;
    }

    pub fn get_dcyc(&self) -> u32 {
        self.dcyc
    }
}

#[cfg(test)]
mod test {
    macro_rules! generator_assert {
        ($f:ident, $e:expr) => {
            let (rx, mut generator) = create_generator();

            generator.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    fn create_generator() -> (::std::sync::mpsc::Receiver<String>, ::generator::Generator) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(
            format!("{}", addr.ip()).as_str(),
            addr.port()
        );

        (rx, ::generator::Generator::new(socket))
    }

    #[test]
    fn test_start() {
        generator_assert!(start, "OUTPUT1:STATE ON\r\n");
    }

    #[test]
    fn test_stop() {
        generator_assert!(stop, "OUTPUT1:STATE OFF\r\n");
    }

    #[test]
    fn test_is_started() {
        let (_, mut generator) = create_generator();

        assert_eq!(generator.is_started(), false);
        generator.start();
        assert_eq!(generator.is_started(), true);
        generator.stop();
        assert_eq!(generator.is_started(), false);
    }

    #[test]
    fn test_form() {
        let (rx, mut generator) = create_generator();

        generator.set_form("SINE");
        assert_eq!("SOUR1:FUNC SINE\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_amplitude() {
        let (rx, mut generator) = create_generator();

        generator.set_amplitude(-0.9);
        assert_eq!("SOUR1:VOLT -0.9\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_frequency() {
        let (rx, mut generator) = create_generator();

        generator.set_frequency(500);
        assert_eq!("SOUR1:FREQ:FIX 500\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_frequency() {
        let (_, generator) = create_generator();

        assert_eq!(generator.get_frequency(), 1000);
    }

    #[test]
    fn test_dcyc() {
        let (rx, mut generator) = create_generator();

        generator.set_dcyc(100);
        assert_eq!("SOUR1:DCYC 100\r\n", rx.recv().unwrap());
    }
}
