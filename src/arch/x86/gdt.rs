#![allow(non_upper_case_globals, non_snake_case)]

use core::mem::offset_of;

use bitfield_struct::bitfield;

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
    pub const KERNEL_CODE_SELECTOR: usize = offset_of!(Gdt, kcode);
    pub const KERNEL_DATA_SELECTOR: usize = offset_of!(Gdt, kdata);
    #[allow(unused)]
    pub const USER_CODE_SELECTOR: usize = offset_of!(Gdt, ucode);
    #[allow(unused)]
    pub const USER_DATA_SELECTOR: usize = offset_of!(Gdt, udata);

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

#[repr(C, packed)]
pub struct Gdtr64 {
    pub size: u16,
    pub offset: u64,
}

impl Gdtr64 {
    pub const GDTR_OFFSET: usize = offset_of!(Gdtr64, offset);
}
