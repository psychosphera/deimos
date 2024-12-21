// TODO: 2m, 4m, and 1g pages

#![allow(unused)]

use bitfield_struct::bitfield;
#[bitfield(u64)]
pub struct Pml5te4k {
    pub present: bool,
    pub rw: bool,
    pub user: bool,
    pub write_through: bool,
    pub cache_disable: bool,
    pub accessed: bool,
    #[bits(3)]
    __: u64,
    #[bits(3)]
    pub available_2: u64,
    #[bits(40)]
    pub pml4_paddr: u64,
    #[bits(11)]
    pub available_3: u64,
    pub nx: bool,
}

impl Pml5te4k {
    pub const fn init() -> Self {
        Self::from_bits(0)
            .with_present(true)
            .with_rw(false)
            .with_user(false)
            .with_write_through(false)
            .with_cache_disable(false)
            .with_accessed(false)
            .with_nx(false)
    }
}

#[bitfield(u64)]
pub struct Pml4te4k {
    pub present: bool,
    pub rw: bool,
    pub user: bool,
    pub write_through: bool,
    pub cache_disable: bool,
    pub accessed: bool,
    #[bits(3)]
    __: u64,
    #[bits(3)]
    pub available_2: u64,
    #[bits(40)]
    pub pdp_paddr: u64,
    #[bits(11)]
    pub available_3: u64,
    pub nx: bool,
}

impl Pml4te4k {
    pub const fn init() -> Self {
        Self::from_bits(0)
            .with_present(true)
            .with_rw(false)
            .with_user(false)
            .with_write_through(false)
            .with_cache_disable(false)
            .with_accessed(false)
            .with_nx(false)
    }
}

#[bitfield(u64)]
pub struct Pdpte4k {
    pub present: bool,
    pub rw: bool,
    pub user: bool,
    pub write_through: bool,
    pub cache_disable: bool,
    pub accessed: bool,
    #[bits(3)]
    __: u64,
    #[bits(3)]
    pub available_2: u64,
    #[bits(40)]
    pub pd_paddr: u64,
    #[bits(11)]
    pub available_3: u64,
    pub nx: bool,
}

impl Pdpte4k {
    pub const fn init() -> Self {
        Self::from_bits(0)
            .with_present(true)
            .with_rw(false)
            .with_user(false)
            .with_write_through(false)
            .with_cache_disable(false)
            .with_accessed(false)
            .with_nx(false)
    }
}

#[bitfield(u64)]
pub struct Pdte4k {
    pub present: bool,
    pub rw: bool,
    pub user: bool,
    pub write_through: bool,
    pub cache_disable: bool,
    pub accessed: bool,
    #[bits(3)]
    __: u64,
    #[bits(3)]
    pub available_2: u64,
    #[bits(40)]
    pub pt_paddr: u64,
    #[bits(11)]
    pub available_3: u64,
    pub nx: bool,
}

impl Pdte4k {
    const fn init() -> Self {
        Self::from_bits(0)
            .with_present(true)
            .with_rw(false)
            .with_user(false)
            .with_write_through(false)
            .with_cache_disable(false)
            .with_accessed(false)
            .with_nx(false)
    }
}

#[bitfield(u64)]
pub struct Pte {
    pub present: bool,
    pub rw: bool,
    pub user: bool,
    pub write_through: bool,
    pub cache_disable: bool,
    pub accessed: bool,
    pub available: bool,
    pub dirty: bool,
    pub page_size: bool,
    #[bits(3)]
    pub available_2: u64,
    #[bits(40)]
    pub page_paddr: u64,
    #[bits(7)]
    pub available_3: u64,
    #[bits(4)]
    pub mkp: u64,
    pub nx: bool,
}

impl Pte {
    pub const fn init() -> Self {
        Self::from_bits(0)
            .with_present(true)
            .with_rw(false)
            .with_user(false)
            .with_write_through(false)
            .with_cache_disable(false)
            .with_accessed(false)
            .with_page_size(false)
            .with_nx(false)
    }
}

#[repr(align(4096))]
pub struct Pml5Table4k(pub [Pml5te4k; 512]);
#[repr(align(4096))]
pub struct Pml4Table4k(pub [Pml4te4k; 512]);
#[repr(align(4096))]
pub struct PageDirectoryPointerTable4k(pub [Pdpte4k; 512]);
#[repr(align(4096))]
pub struct PageDirectoryTable4k(pub [Pdte4k; 512]);
#[repr(align(4096))]
pub struct PageTable(pub [Pte; 512]);

impl PageTable {
    pub const fn identity() -> Self {
        let mut pt = [Pte::init(); 512];
        let mut i = 0;
        while i < pt.len() {
            pt[i].set_page_paddr((i * 4096) as _);
            pt[i].set_rw(true);
            i += 1;
        }

        Self(pt)
    }
}
