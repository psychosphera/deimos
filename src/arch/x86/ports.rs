use core::{arch::asm, marker::PhantomData};

unsafe fn outb(port: u16, b: u8) {
    unsafe { asm!(
        "out dx, al",
        in("dx") port,
        in("al") b,
    ) }
}

unsafe fn outw(port: u16, w: u16) {
    unsafe { asm!(
        "out dx, ax",
        in("dx") port,
        in("ax") w,
    ) }
}

unsafe fn outd(port: u16, d: u32) {
    unsafe { asm!(
        "out dx, al",
        in("dx") port,
        in("eax") d,
    ) }
}

unsafe fn inb(port: u16) -> u8 {
    let mut b;
    unsafe { asm!(
        "in al, dx",
        in("dx") port,
        out("al") b,
    ) }
    b
}

unsafe fn inw(port: u16) -> u16 {
    let mut w;
    unsafe { asm!(
        "in ax, dx",
        in("dx") port,
        out("ax") w,
    ) }
    w
}

unsafe fn ind(port: u16) -> u32 {
    let mut d;
    unsafe { asm!(
        "in eax, dx",
        in("dx") port,
        out("eax") d,
    ) }
    d
}

pub trait PortTrait {
    fn get(&self) -> u16;
}

trait PortTypestate {}

#[derive(Copy, Clone)]
pub struct Port(u16);

impl PortTrait for Port {
    fn get(&self) -> u16 {
        self.0
    }
}

impl Port {
    const fn new(port: u16) -> Self {
        Self(port)
    }
}

pub trait PortRead: PortTrait {
    unsafe fn read_byte(&self) -> u8 {
        unsafe { inb(self.get()) }
    }

    unsafe fn read_word(&self) -> u16 {
        unsafe { inw(self.get()) }
    }

    unsafe fn read_dword(&self) -> u32 {
        unsafe { ind(self.get()) }
    }
}

pub trait PortWrite: PortTrait {
    unsafe fn write_byte(&self, b: u8) {
        unsafe { outb(self.get(), b) }
    }

    unsafe fn write_word(&self, w: u16) {
        unsafe { outw(self.get(), w) }
    }

    unsafe fn write_dword(&self, d: u32) {
        unsafe { outd(self.get(), d) }
    }
}

#[repr(transparent)]
pub struct PortRO(Port);
impl PortTrait for PortRO {
    fn get(&self) -> u16 {
        self.0.get()
    }
}
impl PortRead for PortRO {}
impl PortRO {
    pub const fn new(port: u16) -> Self {
        Self(Port::new(port))
    }
}

#[repr(transparent)]
pub struct PortWO(Port);
impl PortTrait for PortWO {
    fn get(&self) -> u16 {
        self.0.get()
    }
}
impl PortWrite for PortWO {}
impl PortWO {
    pub const fn new(port: u16) -> Self {
        Self(Port::new(port))
    }
}

#[repr(transparent)]
pub struct PortRW(Port);
impl PortTrait for PortRW {
    fn get(&self) -> u16 {
        self.0.get()
    }
}
impl PortRead for PortRW {}
impl PortWrite for PortRW {}
impl PortRW {
    pub const fn new(port: u16) -> Self {
        Self(Port::new(port))
    }
}