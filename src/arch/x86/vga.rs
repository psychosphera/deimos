use core::sync::atomic::{AtomicIsize, Ordering};

const VGA_BUFFER: *mut u16 = 0x000B8000 as _;
const VGA_BUFFER_LEN: usize = 25 * 80;

pub struct VgaWriter {
    pos: usize,
    buf: &'static mut [u16],
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

        let buf = unsafe { core::slice::from_raw_parts_mut(VGA_BUFFER, VGA_BUFFER_LEN) };
        let pos = 0;
        
        VGA_NESTING.fetch_add(1, Ordering::Relaxed);

        Some(Self {
            buf,
            pos,
        })
    }

    pub fn clear(&mut self, background_color: VgaColor) {
        for c in self.buf.iter_mut() {
            *c = ((background_color as u16) << 12) & 0xF000;
        }

        self.pos = 0;
    }

    pub fn putc(&mut self, c: u8, text_color: VgaColor) {
        if self.pos >= self.buf.len() || c < b' ' || c >= 127 {
            return;
        }

        self.buf[self.pos] = self.buf[self.pos] & 0xF000 | c as u16 | (((text_color as u16) << 8) & 0x0F00);
        self.pos += 1;
    }
}

impl Drop for VgaWriter {
    fn drop(&mut self) {
        VGA_NESTING.fetch_sub(1, Ordering::Relaxed);
    }
}