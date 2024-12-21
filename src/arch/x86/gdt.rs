#![allow(non_upper_case_globals, non_snake_case)]

use bitfield_struct::bitfield;

use crate::sizeof;

#[bitfield(u64)]
struct GdtSegmentSelector {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_mid: u8,
    pub accessed: bool,
    pub rw: bool,
    pub direction_conforming: bool,
    pub executable: bool,
    pub type_: bool,
    #[bits(2)]
    pub dpl: u64,
    pub present: bool,
    #[bits(4)]
    pub limit_high: u64,
    pub reserved: bool,
    pub long_mode: bool,
    pub size: bool,
    pub granularity: bool,
    pub base_high: u8,
}

impl GdtSegmentSelector {
    const NULL: Self = GdtSegmentSelector::from_bits(0);

    const BASE: Self = GdtSegmentSelector::from_bits(0)
        .with_limit_low(0xFFFF)
        .with_limit_high(0xF)
        .with_rw(true)
        .with_direction_conforming(false)
        .with_type_(true)
        .with_present(true)
        .with_long_mode(true)
        .with_size(true)
        .with_granularity(true);

    const CODE: Self = Self::BASE.with_executable(true);
    const DATA: Self = Self::BASE.with_executable(false);
    const KERNEL_CODE: Self = Self::CODE.with_dpl(0);
    const KERNEL_DATA: Self = Self::DATA.with_dpl(0);
    const USER_CODE: Self = Self::CODE.with_dpl(3);
    const USER_DATA: Self = Self::DATA.with_dpl(3);
}

#[repr(C, packed)]
pub struct Gdt {
    null: GdtSegmentSelector,
    kcode: GdtSegmentSelector,
    kdata: GdtSegmentSelector,
    ucode: GdtSegmentSelector,
    udata: GdtSegmentSelector,
}

impl Gdt {
    pub const fn new() -> Self {
        Self {
            null: GdtSegmentSelector::NULL,
            kcode: GdtSegmentSelector::KERNEL_CODE,
            kdata: GdtSegmentSelector::KERNEL_DATA,
            ucode: GdtSegmentSelector::USER_CODE,
            udata: GdtSegmentSelector::USER_DATA,
        }
    }
}

#[used]
#[unsafe(no_mangle)]
pub static GDT: Gdt = Gdt::new();

#[repr(C, packed)]
pub struct Gdtr64 {
    pub size: u16,
    pub offset: u64,
}

#[used]
#[unsafe(no_mangle)]
pub static mut GDTR: Gdtr64 = Gdtr64 {
    size: sizeof!(Gdt) as u16 - 1,
    offset: 0,
};
