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
