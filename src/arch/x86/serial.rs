use super::ports::{PortRO, PortRW, PortRead, PortWO, PortWrite};

const COM1_PORT_BASE: u16 = 0x03F8;
const COM2_PORT_BASE: u16 = 0x02F8;

#[derive(Debug)]
pub enum ComInitError {
    FaultyHardware,
}

#[derive(Clone)]
pub struct Com<const BASE: u16 = COM1_PORT_BASE> {}

impl<const BASE: u16> Com<BASE> {
    const RX_OFFSET: u16 = 0;
    const TX_OFFSET: u16 = 0;
    const DIV_LSB_OFFSET: u16 = 0;
    const INT_ENABLE_OFFSET: u16 = 1;
    const DIV_MSB_OFFSET: u16 = 1;
    const INT_ID_OFFSET: u16 = 2;
    const FIFO_CTRL_OFFSET: u16 = 2;
    const LINE_CTRL_OFFSET: u16 = 3;
    const MODEM_CTRL_OFFSET: u16 = 4;
    const LINE_STATUS_OFFSET: u16 = 5;
    const MODEM_STATUS_OFFSET: u16 = 6;
    const SCRATCH_OFFSET: u16 = 7;

    pub const RX: PortRO = PortRO::new(BASE + Self::RX_OFFSET);
    pub const TX: PortWO = PortWO::new(BASE + Self::TX_OFFSET);
    pub const DIV_LSB: PortRW = PortRW::new(BASE + Self::DIV_LSB_OFFSET);
    pub const INT_ENABLE: PortRW = PortRW::new(BASE + Self::INT_ENABLE_OFFSET);
    pub const DIV_MSB: PortRW = PortRW::new(BASE + Self::DIV_MSB_OFFSET);
    pub const INT_ID: PortRO = PortRO::new(BASE + Self::INT_ID_OFFSET);
    pub const FIFO_CTRL: PortWO = PortWO::new(BASE + Self::FIFO_CTRL_OFFSET);
    pub const LINE_CTRL: PortRW = PortRW::new(BASE + Self::LINE_CTRL_OFFSET);
    pub const MODEM_CTRL: PortRW = PortRW::new(BASE + Self::MODEM_CTRL_OFFSET);
    pub const LINE_STATUS: PortRO = PortRO::new(BASE + Self::LINE_STATUS_OFFSET);
    pub const MODEM_STATUS: PortRO = PortRO::new(BASE + Self::MODEM_STATUS_OFFSET);
    pub const SCRATCH: PortRW = PortRW::new(BASE + Self::SCRATCH_OFFSET);

    /// Safety: Since [`BASE`] is an arbitrary constant, it may not be a valid
    /// port, and usage could clobber other hardware.
    pub const unsafe fn new() -> Self {
        Self {}
    }

    pub fn init(&self) -> Result<(), ComInitError> {
        unsafe {
            Self::INT_ENABLE.write_byte(0x00);
            Self::LINE_CTRL.write_byte(0x80);
            Self::DIV_LSB.write_byte(3);
            Self::DIV_MSB.write_byte(0);
            Self::LINE_CTRL.write_byte(0x03);
            Self::FIFO_CTRL.write_byte(0xC7);
            Self::MODEM_CTRL.write_byte(0x0B);
            Self::MODEM_CTRL.write_byte(0x1E);
            
            Self::TX.write_byte(0xEE);

            if Self::RX.read_byte() != 0xEE {
                return Err(ComInitError::FaultyHardware);
            }

            Self::MODEM_CTRL.write_byte(0x0F);
            Ok(())
        }
    } 

    pub fn putc(&self, c: u8) {
        unsafe { Self::TX.write_byte(c) };
    }

    pub fn getc(&self) -> u8 {
        unsafe { Self::RX.read_byte() }
    }
}

pub const unsafe fn com1() -> Com<COM1_PORT_BASE> {
    unsafe { Com::new() }
}

pub const unsafe fn com2() -> Com<COM2_PORT_BASE> {
    unsafe { Com::new() }
}