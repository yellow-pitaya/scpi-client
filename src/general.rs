use Module;
use socket::Socket;

#[derive(Clone)]
pub struct General {
    socket: Socket,
}

impl ::Module for General {
    fn new(socket: Socket) -> Self {
        General {
            socket,
        }
    }
}

impl General {
    /**
     * Initializes the library. It must be called first, before any other
     * library method.
     */
    pub fn init(&self) {
        self.socket.send("RP:INit");
    }

    /**
     * Resets all modules.
     */
    pub fn reset(&self) {
        self.socket.send("RP:REset");
    }

    /**
     * Releases the library resources. It must be called last, after library is
     * not used anymore. Typically before application exits.
     */
    pub fn release(&self) {
        self.socket.send("RP:RELease");
    }

    /**
     * Load specified bitstream version (0.93 or 0.94) in the fpga.
     */
    pub fn fpga_load_bitstream(&self, version: f32) {
        self.socket.send(format!("RP:FPGABITREAM {}", version));
    }

    /**
     * Enable digital loop.
     *
     * This internally connect output to input.
     */
    pub fn enable_digital_loop(&self) {
        self.socket.send("RP:DIG:LOop");
    }
}

#[cfg(test)]
#[cfg(feature = "mock")]
mod test {
    macro_rules! general_assert {
        ($f:ident, $e:expr) => {
            let (rx, rp) = ::test::create_client();

            rp.general.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    #[test]
    fn test_init() {
        general_assert!(init, "RP:INit\r\n");
    }

    #[test]
    fn test_reset() {
        general_assert!(reset, "RP:REset\r\n");
    }

    #[test]
    fn test_release() {
        general_assert!(release, "RP:RELease\r\n");
    }

    #[test]
    fn test_fpga_load_bitstream() {
        let (rx, rp) = ::test::create_client();

        rp.general.fpga_load_bitstream(0.93);
        assert_eq!("RP:FPGABITREAM 0.93\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_enable_digital_loop() {
        general_assert!(enable_digital_loop, "RP:DIG:LOop\r\n");
    }
}
