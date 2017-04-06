#[macro_use]
extern crate log;

mod acquire;
mod generator;
mod socket;
mod trigger;

pub struct Redpitaya {
    pub acquire: acquire::Acquire,
    pub generator: generator::Generator,
    pub trigger: trigger::Trigger,
}

impl Redpitaya {
    pub fn new(ip: &str, port: u16) -> Redpitaya {
        let socket = socket::Socket::new(ip, port);

        Redpitaya {
            acquire: acquire::Acquire::new(socket.clone()),
            generator: generator::Generator::new(socket.clone()),
            trigger: trigger::Trigger::new(socket.clone()),
        }
    }
}
