use socket::Socket;

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

    pub fn set_units(&mut self, unit: &str) {
        self.socket.send(format!("ACQ:DATA:UNITS {}", unit));
    }

    pub fn set_decimation(&mut self, decimation: u8) {
        self.socket.send(format!("ACQ:DEC {}", decimation));
    }

    pub fn get_decimation(&mut self) -> u8 {
        self.socket.send("ACQ:DEC?");

        self.socket.receive()
            .parse()
            .unwrap()
    }

    pub fn get_data(&mut self) -> String {
        self.socket.send("ACQ:SOUR1:DATA?");

        self.socket.receive()
    }
}
