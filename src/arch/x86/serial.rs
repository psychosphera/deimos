use bitfield_struct::bitfield;
use bitflags::bitflags;

use super::ports::{PortRO, PortRW, PortRead, PortReadCustom, PortWO, PortWrite, PortWriteCustom};

const COM1_PORT_BASE: u16 = 0x03F8;
const COM2_PORT_BASE: u16 = 0x02F8;

#[derive(Debug)]
pub enum ComInitError {
    FaultyHardware,
}

#[derive(Clone)]
pub struct Com<const BASE: u16 = COM1_PORT_BASE> {}

pub struct IntEnableReg(PortRW);

#[bitfield(u8)]
pub struct IntEnableFlags {
    pub rx_available: bool,
    pub tx_empty: bool,
    pub rx_line_status: bool,
    pub modem_status: bool,
    #[bits(4)]
    __: u8,
}

impl PortReadCustom for IntEnableReg {
    type Item = IntEnableFlags;

    unsafe fn read(&self) -> Self::Item {
        IntEnableFlags::from_bits(unsafe { self.0.read_byte() })
    }
}

impl PortWriteCustom for IntEnableReg {
    type Item = IntEnableFlags;

    unsafe fn write(&self, item: Self::Item) {
        unsafe { self.0.write_byte(item.into_bits()) };
    }
}

pub struct LineCtrlReg(PortRW);

#[bitfield(u8)]
pub struct LineCtrlFlags {
    #[bits(2)]
    pub data_bits: u8,
    pub stop_bits: bool,
    #[bits(3)]
    pub parity_bits: u8,
    pub break_enable: bool,
    pub dlab: bool,
}

impl PortReadCustom for LineCtrlReg {
    type Item = LineCtrlFlags;

    unsafe fn read(&self) -> Self::Item {
        LineCtrlFlags::from_bits(unsafe { self.0.read_byte() })
    }
}

impl PortWriteCustom for LineCtrlReg {
    type Item = LineCtrlFlags;

    unsafe fn write(&self, item: Self::Item) {
        unsafe { self.0.write_byte(item.into_bits()) };
    }
}

pub struct FifoCtrlReg(PortWO);

#[bitfield(u8)]
pub struct FifoCtrlFlags {
    pub enable: bool,
    pub clear_rx: bool,
    pub clear_tx: bool,
    pub dma_mode_select: bool,
    #[bits(2)]
    __: u8,
    #[bits(2)]
    pub interrupt_trigger_level: u8,
}

impl PortWriteCustom for FifoCtrlReg {
    type Item = FifoCtrlFlags;

    unsafe fn write(&self, item: Self::Item) {
        unsafe { self.0.write_byte(item.into_bits()) };
    }
}

pub struct ModemCtrlReg(PortRW);

#[bitfield(u8)]
pub struct ModemCtrlFlags {
    pub dtr: bool,
    pub rts: bool,
    pub out1: bool,
    pub irq_enable: bool,
    pub loopback: bool,
    #[bits(3)]
    __: u8,
}

impl PortReadCustom for ModemCtrlReg {
    type Item = ModemCtrlFlags;

    unsafe fn read(&self) -> Self::Item {
        ModemCtrlFlags::from_bits(unsafe { self.0.read_byte() })
    }
}

impl PortWriteCustom for ModemCtrlReg {
    type Item = ModemCtrlFlags;

    unsafe fn write(&self, item: Self::Item) {
        unsafe { self.0.write_byte(item.into_bits()) };
    }
}

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
    pub const INT_ENABLE: IntEnableReg = IntEnableReg(PortRW::new(BASE + Self::INT_ENABLE_OFFSET));
    pub const DIV_MSB: PortRW = PortRW::new(BASE + Self::DIV_MSB_OFFSET);
    pub const INT_ID: PortRO = PortRO::new(BASE + Self::INT_ID_OFFSET);
    pub const FIFO_CTRL: FifoCtrlReg = FifoCtrlReg(PortWO::new(BASE + Self::FIFO_CTRL_OFFSET));
    pub const LINE_CTRL: LineCtrlReg = LineCtrlReg(PortRW::new(BASE + Self::LINE_CTRL_OFFSET));
    pub const MODEM_CTRL: ModemCtrlReg = ModemCtrlReg(PortRW::new(BASE + Self::MODEM_CTRL_OFFSET));
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
            Self::INT_ENABLE.write(IntEnableFlags::new().with_rx_available(false).with_tx_empty(false).with_rx_line_status(false).with_modem_status(false));
            Self::LINE_CTRL.write(LineCtrlFlags::new().with_dlab(true));
            Self::DIV_LSB.write_byte(3);
            Self::DIV_MSB.write_byte(0);
            Self::LINE_CTRL.write(LineCtrlFlags::new().with_dlab(false).with_parity_bits(0).with_stop_bits(false).with_data_bits(3));
            Self::FIFO_CTRL.write(FifoCtrlFlags::new().with_enable(true).with_clear_rx(true).with_clear_tx(true).with_interrupt_trigger_level(3));
            Self::MODEM_CTRL.write(ModemCtrlFlags::new().with_dtr(true).with_rts(true).with_irq_enable(true));
            Self::MODEM_CTRL.write(ModemCtrlFlags::new().with_rts(true).with_irq_enable(true).with_out1(true).with_loopback(true));
            
            Self::TX.write_byte(0xEE);

            if Self::RX.read_byte() != 0xEE {
                return Err(ComInitError::FaultyHardware);
            }

            Self::MODEM_CTRL.write(ModemCtrlFlags::new().with_dtr(true).with_rts(true).with_out1(true).with_irq_enable(true).with_loopback(false));
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

pub type COM1 = Com<COM1_PORT_BASE>;
pub type COM2 = Com<COM2_PORT_BASE>;

pub const unsafe fn com1() -> COM1 {
    unsafe { Com::new() }
}

pub const unsafe fn com2() -> COM2 {
    unsafe { Com::new() }
}