#![no_std]
#![no_main]
#![feature(never_type)]

#![allow(non_camel_case_types)]

mod arch;
#[macro_use]
mod common;
mod multiboot;

use core::{arch::{asm, global_asm}, panic::PanicInfo};

use arch::x86::pages::{PageDirectoryPointerTable4k, PageDirectoryTable4k, PageTable, Pdpte4k, Pdte4k, Pml4Table4k, Pml4te4k, Pml5Table4k, Pml5te4k};
use common::LinkerSymbol;

unsafe extern "C" {
    static KERNEL_START: LinkerSymbol;
    static KERNEL_END: LinkerSymbol;
}

#[repr(align(16))]
struct InitStack([u8; 16384]);
impl InitStack {
    const fn new() -> Self {
        Self([0u8; 16384])
    }
}

#[used]
#[unsafe(no_mangle)]
static mut INIT_STACK: InitStack = InitStack::new();

#[used]
#[unsafe(no_mangle)]
static mut INIT_PML5T: Pml5Table4k = Pml5Table4k([Pml5te4k::new(); 512]);
#[used]
#[unsafe(no_mangle)]
static mut INIT_PML4T: Pml4Table4k = Pml4Table4k([Pml4te4k::new(); 512]);
#[used]
#[unsafe(no_mangle)]
static mut INIT_PDPT:  PageDirectoryPointerTable4k = PageDirectoryPointerTable4k([Pdpte4k::new(); 512]);
#[used]
#[unsafe(no_mangle)]
static mut INIT_PDT:   PageDirectoryTable4k = PageDirectoryTable4k([Pdte4k::new(); 512]);
#[used]
#[unsafe(no_mangle)]
static mut INIT_PT:    PageTable = PageTable::identity();


global_asm!("
    .code32

    .global _start

    // sets up a basic page table that identity maps the first 2mb of memory
    // intended to be replaced with a proper page table by the kernel
    // assumes kernel KERNEL_END < 2mb
    init_paging:
        // assumes INIT_PT is already initialized in Rust

        mov eax, INIT_PT
        or eax, 0x00000003 // set Present and RW bits
        or [INIT_PDT], eax

        mov eax, INIT_PDT
        or eax, 0x00000003 // set Present and RW bits
        or [INIT_PDPT], eax
        
        mov eax, INIT_PDPT
        or eax, 0x00000003 // set Present and RW bits
        or [INIT_PML4T], eax

        mov eax, INIT_PML4T
        or eax, 0x00000003 // set Present and RW bits
        or [INIT_PML5T], eax

        // clear CR0.PG in case it's enabled for some reason
        mov ebx, cr0
        and ebx, 0x7FFFFFFF
        mov cr0, ebx

        // set CR4.PAE
        mov eax, cr4
        or eax, 0x00000020
        mov cr4, eax

        // set EFER.LME
        mov ecx, 0xC0000080
        rdmsr
        or ecx, 0x00000100
        wrmsr

        mov eax, INIT_PML4T
        mov cr3, eax

        // set CR0.PG
        or ebx, 0x80000000
        mov cr0, ebx

        ret

    // sets GDTR.base to addr_of GDT and loads GDT
    init_gdt: 
        // assumes GDT is loaded in the lower 4GB of memory
        mov eax, GDT
        mov [GDTR+2], eax
        lgdt [GDTR]
        ret

    _start:
        // multiboot2 doesn't guarantee us a stack, so we have to make one
        mov esp, INIT_STACK
        // assumes stack size is 16384
        add esp, 16384
        push eax
        push ebx
        push ecx
        call init_gdt
        call init_paging
        // assumes kernal data selector is 0x0010
        mov ax, 0x0010
        mov ds, ax
        mov es, ax 
        mov fs, ax
        mov gs, ax
        pop ecx
        pop ebx
        pop eax

        // assumes kernal code selector is 0x0008
        ljmp 0x0008, offset kernel_main
");

#[unsafe(no_mangle)]
extern "C" fn kernel_main() -> ! {
    loop {
        unsafe { asm!("
            cli
            hlt
        ") };
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("
            cli
            hlt
        ") };
    }
}