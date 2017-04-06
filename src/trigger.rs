use socket::Socket;

pub struct Trigger {
    socket: Socket,
}

impl Trigger {
    pub fn new(socket: Socket) -> Self {
        Trigger {
            socket: socket,
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.socket.send(format!("ACQ:TRIG:LEV {}", level));
    }

    pub fn enable(&mut self, source: &str) {
        self.socket.send(format!("ACQ:TRIG {}", source));
    }

    pub fn set_delay(&mut self, delay: u8) {
        self.socket.send(format!("ACQ:TRIG:DLY {}", delay));
    }
}
