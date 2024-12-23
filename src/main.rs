#![no_std]
#![no_main]
#![feature(never_type)]
#![allow(non_camel_case_types)]

mod arch;
#[macro_use]
mod common;
mod multiboot2;

use core::{
    arch::{asm, global_asm}, hint::black_box, panic::PanicInfo, ptr::addr_of
};

use arch::x86::{gdt::{Gdt, Gdtr64}, pages::{
    PageDirectoryPointerTable4k, PageDirectoryTable4k, PageTable, Pdpte4k, Pdte4k, Pml4Table4k,
    Pml4te4k, Pml5Table4k, Pml5te4k,
}};
use multiboot2::{Multiboot2Header, Multiboot2Info, MULTIBOOT2_LOAD_MAGIC};
use common::LinkerSymbol;

unsafe extern "C" {
    static KERNEL_START: LinkerSymbol;
    static KERNEL_END: LinkerSymbol;
}

fn kernel_size() -> usize {
    addr_of!(KERNEL_END) as usize - addr_of!(KERNEL_START) as usize
}

#[repr(align(16))]
struct InitStack(#[allow(unused)] [u8; InitStack::SIZE]);
impl InitStack {
    const SIZE: usize = 16384;

    const fn new() -> Self {
        Self([0u8; Self::SIZE])
    }
}

#[used]
#[unsafe(link_section = ".multiboot2")]
static mut MULTIBOOT2_HEADER: Multiboot2Header = Multiboot2Header::new();

#[used]
#[unsafe(no_mangle)]
static mut INIT_STACK: InitStack = InitStack::new();

#[used]
#[unsafe(no_mangle)]
static mut GDT: Gdt = Gdt::new();

#[used]
#[unsafe(no_mangle)]
static mut GDTR: Gdtr64 = Gdtr64 {
    size: size_of!(Gdt) as u16 - 1,
    offset: 0,
};

#[used]
#[unsafe(no_mangle)]
static mut INIT_PML5T: Pml5Table4k = Pml5Table4k([Pml5te4k::from_bits(0); 512]);
#[used]
#[unsafe(no_mangle)]
static mut INIT_PML4T: Pml4Table4k = Pml4Table4k([Pml4te4k::from_bits(0); 512]);
#[used]
#[unsafe(no_mangle)]
static mut INIT_PDPT: PageDirectoryPointerTable4k =
    PageDirectoryPointerTable4k([Pdpte4k::from_bits(0); 512]);
#[used]
#[unsafe(no_mangle)]
static mut INIT_PDT: PageDirectoryTable4k = PageDirectoryTable4k([Pdte4k::from_bits(0); 512]);
#[used]
#[unsafe(no_mangle)]
static mut INIT_PT: PageTable = PageTable::identity();

global_asm!(
    include_str!("boot.s"),
    KERNEL_CODE_SELECTOR = const Gdt::KERNEL_CODE_SELECTOR,
    KERNEL_DATA_SELECTOR = const Gdt::KERNEL_DATA_SELECTOR,
    PAGE_PRESENT         = const 0x00000001,
    PAGE_RW              = const 0x00000002,
    CR0_PG               = const 0x80000000u32 as i32,
    CR4_PAE              = const 0x00000020,
    EFER                 = const 0xC0000080u32 as i32,
    EFER_LME             = const 0x00000100,
    INIT_STACK_SIZE      = const InitStack::SIZE,
    GDTR_OFFSET          = const Gdtr64::GDTR_OFFSET,
);

#[unsafe(no_mangle)]
extern "C" fn kernel_main(magic: u32, multiboot2_info: *mut Multiboot2Info) -> ! {
    black_box(&raw const MULTIBOOT2_HEADER);
    black_box(&raw const INIT_STACK);
    black_box(&raw const INIT_PML5T);
    black_box(&raw const INIT_PML4T);
    black_box(&raw const INIT_PDPT);
    black_box(&raw const INIT_PDT);
    black_box(&raw const INIT_PT);
    black_box(&raw const GDT);
    black_box(&raw const GDTR);

    assert!(magic == MULTIBOOT2_LOAD_MAGIC);
    assert!(!multiboot2_info.is_null());
    assert!(kernel_size() <= 2 * 1024 * 1024);

    let s = b"Hello, World!";
    for (i, c) in s.iter().enumerate() {
        unsafe { *(0x000B8000 as *mut u16).add(i) = *c as u16 | 0x0F00 };
    }

    loop {
        unsafe {
            asm!(
                "
            cli
            hlt
        "
            )
        };
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    let s = b"Panicked!";
    for (i, c) in s.iter().enumerate() {
        unsafe { *(0x000B8000 as *mut u16).add(i) = *c as u16 | 0x0400 };
    }

    loop {
        unsafe {
            asm!(
                "
            cli
            hlt
        "
            )
        };
    }
}
