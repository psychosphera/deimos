use core::sync::atomic::{AtomicIsize, Ordering};

use volatile::VolatileRef;

use super::ports::{PortRW, PortRead, PortWrite};

const VGA_BUFFER: *mut u16 = 0x000B8000 as _;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_BUFFER_LEN: usize = VGA_WIDTH * VGA_HEIGHT;
const TAB_LEN: usize = 4;
const CRTC_ADDR_PORT: u16 = 0x03D4;
const CRTC_DATA_PORT: u16 = 0x03D5;
const CURSOR_HEIGHT: u8 = 0;

// not working
struct VgaCursor {
    enabled: bool,
}

impl VgaCursor {
    const CRTC_ADDR: PortRW = PortRW::new(CRTC_ADDR_PORT);
    const CRTC_DATA: PortRW = PortRW::new(CRTC_DATA_PORT);

    pub unsafe fn new() -> Self { 
        Self {
            enabled: false,
        } 
    }

    /// [`cursor_height`] must be <= 15, no-op if it's not
    pub fn enable(&mut self, cursor_height: u8) {
        if cursor_height > 15 {
            return;
        }

        if self.enabled {
            return;
        }

        unsafe {
            Self::CRTC_ADDR.write_byte(0x0A);
            Self::CRTC_DATA.write_byte(Self::CRTC_DATA.read_byte() & 0xC0);

            self.set_height(cursor_height);
        }

        self.enabled = true;
    }

    pub fn disable(&mut self) {
        if !self.enabled {
            return;
        }

        unsafe {
            Self::CRTC_ADDR.write_byte(0x0A);
            Self::CRTC_DATA.write_byte(0x20);
        }

        self.enabled = false;
    }

    pub fn set_height(&self, cursor_height: u8) {
        unsafe {
            Self::CRTC_ADDR.write_byte(0x0B);
            Self::CRTC_DATA.write_byte(Self::CRTC_DATA.read_byte() & 0xE0 | cursor_height);
        }
    }

    pub fn update(&self, pos: u16) {
        unsafe {
            Self::CRTC_ADDR.write_byte(0x0F);
            Self::CRTC_DATA.write_byte(pos as u8);

            Self::CRTC_ADDR.write_byte(0x0E);
            Self::CRTC_DATA.write_byte(((pos >> 8) & 0xFF) as u8);
        }
    }
}

pub struct VgaWriter {
    pos: usize,
    buf: VolatileRef<'static, [u16]>,
    cursor: VgaCursor,
}

#[derive(Copy, Clone, Debug)]
#[repr(u16)]
pub enum VgaColor {
    BLACK = 0x0000,
    BLUE = 0x0001,
    GREEN = 0x0002,
    CYAN = 0x0003,
    RED = 0x0004,
    MAGENTA = 0x0005,
    BROWN = 0x0006,
    LIGHT_GREY = 0x0007,
    DARK_GREY = 0x0008,
	LIGHT_BLUE = 0x0009,
	LIGHT_GREEN = 0x000A,
	LIGHT_CYAN = 0x000B,
	LIGHT_RED = 0x000C,
	LIGHT_MAGENTA = 0x000D,
	LIGHT_BROWN = 0x000E,
	WHITE = 0x000F,
}

static VGA_NESTING: AtomicIsize = AtomicIsize::new(0);

impl VgaWriter {
    /// Returns [`None`] if another [`VgaWriter`] exists.
    /// 
    /// Safety: assumes VGA buffer is 80x25 and lies at [`VGA_BUFFER`].
    pub unsafe fn new() -> Option<Self> {
        if VGA_NESTING.load(Ordering::Relaxed) > 0 {
            return None;
        }

        let buf = VolatileRef::from_mut_ref(unsafe { core::slice::from_raw_parts_mut(VGA_BUFFER, VGA_BUFFER_LEN) });
        let pos = 0;
        let cursor = unsafe { VgaCursor::new() };
        
        VGA_NESTING.fetch_add(1, Ordering::Relaxed);

        Some(Self {
            buf,
            pos,
            cursor,
        })
    }

    fn read(&mut self, index: usize) -> u16 {
        assert!(index < VGA_BUFFER_LEN);

        let c = unsafe { self.buf.as_mut_ptr().as_raw_ptr().cast::<u16>().add(index) };
        unsafe { c.read_volatile() }
    }

    fn write(&mut self, index: usize, val: u16) {
        assert!(index < VGA_BUFFER_LEN);

        let c = unsafe { self.buf.as_mut_ptr().as_raw_ptr().cast::<u16>().add(index) };
        unsafe { c.write_volatile(val) };
    }

    pub fn clear(&mut self, background_color: VgaColor) {
        for i in 0..VGA_BUFFER_LEN {
            let c = unsafe { self.buf.as_mut_ptr().as_raw_ptr().cast::<u16>().add(i) };
            self.write(i, ((background_color as u16) << 12) & 0xF000);
        }

        self.pos = 0;
    }

    pub fn enable_cursor(&mut self) {
        self.cursor.enable(CURSOR_HEIGHT);
        self.cursor.update(self.pos as _);
    }

    pub fn disable_cursor(&mut self) {
        self.cursor.disable();
    }

    pub fn putc(&mut self, c: u8, text_color: VgaColor) {
        self.putc_internal(c, text_color);
        self.cursor.update(self.pos as _);
        
    }

    fn putc_internal(&mut self, c: u8, text_color: VgaColor) {
        if !Self::is_printable(c) {
            return;
        }

        if self.pos >= VGA_BUFFER_LEN {
            self.scroll();
        }

        if !Self::is_trivially_printable(c) {
            self.put_control_char(c, text_color);
            return;
        }

        let val = self.read(self.pos) & 0xF000 | c as u16 | (((text_color as u16) << 8) & 0x0F00);
        self.write(self.pos, val);
        self.pos += 1;
    }

    pub fn puts(&mut self, s: impl AsRef<str>, text_color: VgaColor) {
        for c in s.as_ref().chars() {
            if c.is_ascii() {
                self.putc_internal(c as u8, text_color);
            }
        }

        self.cursor.update(self.pos as _);
    }

    fn put_control_char(&mut self, c: u8, text_color: VgaColor) {
        match c {
            b'\t' => for _ in 0..TAB_LEN {
                self.putc(b' ', text_color);
            },
            b'\n' => {
                self.pos += VGA_WIDTH - self.pos % VGA_WIDTH;
            },
            0x08 => { // backspace
                let val= self.read(self.pos) & 0xF000 | b' ' as u16 | (((text_color as u16) << 8) & 0x0F00);
                self.write(self.pos, val);
            },
            _ => {}
        }
    }

    fn is_trivially_printable(c: u8) -> bool {
        c >= b' ' && c < 127
    }

    fn is_printable(c: u8) -> bool {
        c != b'\0' && c < 127
    }

    fn scroll(&mut self) {
        unsafe { core::ptr::copy(VGA_BUFFER.add(VGA_HEIGHT), VGA_BUFFER, self.pos) };
        self.pos -= VGA_WIDTH;
    }


}

impl Drop for VgaWriter {
    fn drop(&mut self) {
        VGA_NESTING.fetch_sub(1, Ordering::Relaxed);
    }
}