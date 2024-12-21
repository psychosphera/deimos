#[macro_export]
macro_rules! sizeof {
    ($t:ty) => {
        core::mem::size_of::<$t>()
    };
    ($e:expr) => {
        core::mem::size_of_val($e)
    };
}

#[repr(C)]
#[allow(unused)]
pub struct LinkerSymbol {}
