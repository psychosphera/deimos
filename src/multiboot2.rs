use core::marker::PhantomData;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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
    pad: u32,
}

impl Multiboot2FramebufferTag {
    const fn new() -> Self {
        Self {
            base: Multiboot2TagBase { type_: Multiboot2TagType::FRAMEBUFFER, flags: 0, size: size_of!(Self) as u32 },
            width: 1024,
            height: 768,
            depth: 32,
            pad: 0,
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
pub struct Multiboot2InfoHeader {
    pub total_size: u32,
    reserved: u32,
}

#[repr(C, packed)]
pub struct Multiboot2InfoTag {
    pub type_: u32,
    pub size: u32,
}

#[derive(FromPrimitive)]
#[repr(u32)]
pub enum Multiboot2InfoTagType {
    BOOT_CMDLINE = 1,
    BOOTLOADER_NAME = 2,
    MODULES = 3,
    BASIC_MEMORY_INFO = 4,
    BIOS_BOOT_DEVICE = 5,
    MEMORY_MAP = 6,
    VBE_INFO = 7,
    FRAMEBUFFER_INFO = 8,
    ELF_SYMBOLS = 9,
    APM_TABLE = 10,
    EFI_SYSTEM_TABLE_PTR_32 = 11,
    EFI_SYSTEM_TABLE_PTR_64 = 12,
    SMBIOS_TABLES = 13,
    ACPI_1_0_RSDP = 14,
    ACPI_2_0_RSDP = 15,
    NETWORKING_INFO = 16,
    EFI_MEMORY_MAP = 17,
    EFI_BOOT_SERVICES_NOT_TERMINATED = 18,
    EFI_IMAGE_HANDLE_PTR_32 = 19,
    EFI_IMAGE_HANDLE_PTR_64 = 20,
    IMAGE_LOAD_BASE_PADDR = 21,
}

pub enum Multiboot2Info<'a> {
    MemoryMap(&'a [Multiboot2MemoryMapEntry]),
    Unimplemented(Multiboot2InfoTagType),
}

#[repr(C, packed)]
struct Multiboot2MemoryMapTag {
    base: Multiboot2InfoTag,
    entry_size: u32,
    entry_version: u32,
}

#[repr(C, packed)]
pub struct Multiboot2MemoryMapEntry {
    pub base_paddr: u64,
    pub length: u64,
    pub type_: u32,
    reserved: u32,
}

pub struct Multiboot2InfoIter<'a> {
    begin_ptr: *const u8,
    current_ptr: *const Multiboot2InfoTag,
    total_size: u32,
    __p: PhantomData<&'a ()>,
}

impl<'a> Iterator for Multiboot2InfoIter<'a> {
    type Item = Multiboot2Info<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_ptr as usize >= unsafe { self.begin_ptr.add(self.total_size as _) } as usize {
            return None;
        }

        let item = match Multiboot2InfoTagType::from_u32(unsafe {(*self.current_ptr).type_})? {
            Multiboot2InfoTagType::MEMORY_MAP => {
                let tag = self.current_ptr.cast::<Multiboot2MemoryMapTag>();
                let slice = unsafe {
                    let entries_ptr = tag.add(1).cast::<Multiboot2MemoryMapEntry>();
                    let count = ((*tag).base.size as usize - size_of!(Multiboot2MemoryMapTag)) / size_of!(Multiboot2MemoryMapEntry);
                    core::slice::from_raw_parts(entries_ptr, count)
                };
                Multiboot2Info::MemoryMap(slice)
            },
            other => Multiboot2Info::Unimplemented(other),
        };
        let p = unsafe { self.current_ptr.cast::<u8>().add((*self.current_ptr).size as usize) };
        self.current_ptr = unsafe { p.add(p.align_offset(8)).cast() };
        
        Some(item)
    }
}

impl<'a> Multiboot2InfoIter<'a> {
    pub fn new(header: *const Multiboot2InfoHeader) -> Self {
        let tags = unsafe { header.add(1).cast::<Multiboot2InfoTag>() };
        let total_size = unsafe { (*header).total_size };

        Self {
            begin_ptr: tags.cast(),
            current_ptr: tags,
            total_size,
            __p: PhantomData,
        }
    }
}