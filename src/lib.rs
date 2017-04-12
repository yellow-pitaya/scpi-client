#[macro_use]
extern crate log;

pub mod acquire;
pub mod analog;
pub mod burst;
pub mod data;
pub mod digital;
pub mod generator;
pub mod socket;
pub mod trigger;

pub struct Redpitaya {
    pub acquire: acquire::Acquire,
    pub analog: analog::Analog,
    pub burst: burst::Burst,
    pub digital: digital::Digital,
    pub generator: generator::Generator,
    pub trigger: trigger::Trigger,
    pub data: data::Data,
}

impl Redpitaya {
    pub fn new<S>(addr: S) -> Redpitaya
        where S: ::std::net::ToSocketAddrs
    {
        let socket = socket::Socket::new(addr);

        Redpitaya {
            acquire: acquire::Acquire::new(socket.clone()),
            analog: analog::Analog::new(socket.clone()),
            burst: burst::Burst::new(socket.clone()),
            digital: digital::Digital::new(socket.clone()),
            generator: generator::Generator::new(socket.clone()),
            trigger: trigger::Trigger::new(socket.clone()),
            data: data::Data::new(socket.clone()),
        }
    }
}

#[cfg(test)]
mod test {
    use ::std::io::Read;
    use ::std::io::Write;

    pub fn launch_server() -> (::std::net::SocketAddr, ::std::sync::mpsc::Receiver<String>) {
        let addr = next_test_ip4();
        let listener = ::std::net::TcpListener::bind(format!("{}", addr))
            .unwrap();

        let (tx, rx) = ::std::sync::mpsc::channel();

        ::std::thread::spawn(move || {
            loop {
                if let Ok((mut stream, _)) =  listener.accept() {
                    let tx = tx.clone();

                    ::std::thread::spawn(move || {
                        let mut message = String::new();

                        loop {
                            let mut buffer = [0; 1];

                            stream.read(&mut buffer[..])
                                .unwrap();
                            message.push(buffer[0] as char);

                            if buffer[0] == ('\n' as u8) {
                                break;
                            }
                        }

                        match message.as_str() {
                            "ACQ:DEC?\r\n" => {
                                stream.write("1\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:AVG?\r\n" => {
                                stream.write("ON\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:WPOS?\r\n" => {
                                stream.write("1024\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:TPOS?\r\n" => {
                                stream.write("512\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:SOUR1:DATA:STA:END? 10,13\r\n" => {
                                stream.write("{123,231,-231}\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:SOUR1:DATA:STA:N? 10,3\r\n" => {
                                stream.write("{1.2,3.2,-1.2}\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:SOUR1:DATA?\r\n" => {
                                stream.write("{1.2,3.2,-1.2}\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:SOUR1:DATA:OLD:N? 2\r\n" => {
                                stream.write("{3.2,-1.2}\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:SOUR1:DATA:LAT:N? 2\r\n" => {
                                stream.write("{1.2,3.2}\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:BUF:SIZE?\r\n" => {
                                stream.write("16384\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:TRIG:STAT?\r\n" => {
                                stream.write("WAIT\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:TRIG:DLY?\r\n" => {
                                stream.write("2314\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:TRIG:DLY:NS?\r\n" => {
                                stream.write("128ns\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ACQ:TRIG:LEV?\r\n" => {
                                stream.write("123mV\r\n".as_bytes())
                                    .unwrap();
                            },
                            "ANALOG:PIN? AIN1\r\n" => {
                                stream.write("1.34\r\n".as_bytes())
                                    .unwrap();
                            },
                            "DIG:PIN? DIO0_N\r\n" => {
                                stream.write("1\r\n".as_bytes())
                                    .unwrap();
                            },
                            _ => tx.send(message).unwrap(),
                        };
                    });
                }
            }
        });

        (addr, rx)
    }

    static PORT: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::ATOMIC_USIZE_INIT;

    fn next_test_ip4() -> ::std::net::SocketAddr {
        let port = PORT.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u16 + base_port();
        ::std::net::SocketAddr::V4(::std::net::SocketAddrV4::new(::std::net::Ipv4Addr::new(127, 0, 0, 1), port))
    }

    // The bots run multiple builds at the same time, and these builds
    // all want to use ports. This function figures out which workspace
    // it is running in and assigns a port range based on it.
    fn base_port() -> u16 {
        let cwd = ::std::env::current_dir()
            .unwrap();
        let dirs = [
            "32-opt",
            "32-nopt",
            "musl-64-opt",
            "cross-opt",
            "64-opt",
            "64-nopt",
            "64-opt-vg",
            "64-debug-opt",
            "all-opt",
            "snap3",
            "dist",
        ];

        dirs.iter()
            .enumerate()
            .find(|&(_, dir)| cwd.to_str().unwrap().contains(dir))
            .map(|p| p.0)
            .unwrap_or(0) as u16 * 1000 + 19600
    }
}
