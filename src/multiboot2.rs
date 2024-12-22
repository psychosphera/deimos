#[repr(C, packed)]
pub struct Multiboot2Header {
    magic: Multiboot2Magic,
    arch: Multiboot2Arch, 
    header_length: u32,
    checksum: u32,
    //framebuffer_tag: Multiboot2FramebufferTag,
    final_tag: Multiboot2FinalTag,
}

impl Multiboot2Header {
    pub const fn new() -> Self {
        Self {
            magic: Multiboot2Magic::new(),
            arch: Multiboot2Arch::PROTECTED_MODE,
            header_length: size_of!(Multiboot2Header) as u32,
            checksum: (-((Multiboot2Magic::new().0 as u32 + Multiboot2Arch::PROTECTED_MODE as u32 + size_of!(Multiboot2Header) as u32) as i32)) as u32,
            //framebuffer_tag: Multiboot2FramebufferTag::new(),
            final_tag: Multiboot2FinalTag::new(),
        }
    }
}

const MULTIBOOT2_HEADER_MAGIC: u32 = 0xE85250D6;

pub const MULTIBOOT2_LOAD_MAGIC: u32 = 0x36D76289;

#[repr(transparent)]
struct Multiboot2Magic(u32);
impl Multiboot2Magic {
    const fn new() -> Self {
        Self(MULTIBOOT2_HEADER_MAGIC)
    }
}

#[repr(u32)]
enum Multiboot2Arch {
    PROTECTED_MODE = 0,
}

#[repr(u16)]
enum Multiboot2TagType {
    FINAL = 0,
    FRAMEBUFFER = 5,
}

#[repr(C, packed)]
struct Multiboot2TagBase {
    type_: Multiboot2TagType,
    flags: u16,
    size: u32,
}

#[repr(C, packed)]
struct Multiboot2FramebufferTag {
    base: Multiboot2TagBase,
    width: u32,
    height: u32,
    depth: u32,
}

impl Multiboot2FramebufferTag {
    const fn new() -> Self {
        Self {
            base: Multiboot2TagBase { type_: Multiboot2TagType::FRAMEBUFFER, flags: 0, size: size_of!(Self) as u32 },
            width: 1024,
            height: 768,
            depth: 32,
        }
    }
}

#[repr(C, packed)]
struct Multiboot2FinalTag {
    base: Multiboot2TagBase,
}

impl Multiboot2FinalTag {
    const fn new() -> Self {
        Self {
            base: Multiboot2TagBase { type_: Multiboot2TagType::FINAL, flags: 0, size: size_of!(Self) as u32 },
        }
    }
}

#[repr(C, packed)]
pub struct Multiboot2Info {
    
}