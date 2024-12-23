use core::ffi::c_void;

#[macro_export]
macro_rules! size_of {
    ($t:ty) => {
        core::mem::size_of::<$t>()
    };
    ($e:expr) => {
        core::mem::size_of_val($e)
    };
}

#[repr(C)]
pub struct LinkerSymbol {
    __: c_void,
}