macro_rules! vfunc {
    ($start:expr, $t:ty, $index:tt) => {
        {
            use std::mem;
            let addr = unsafe {
                *((*($start as *const usize) + mem::size_of::<usize>() * $index) as *const usize)
            };
            let func: extern "thiscall" fn(*mut c_void) -> $t = unsafe { mem::transmute(addr as *mut c_void) };
            func
        }
    };
    ($start:expr, $t:ty, $index:tt, $($arg:ty),+) => {
        {
            use std::mem;
            let addr = unsafe {
                *((*($start as *const usize) + mem::size_of::<usize>() * $index) as *const usize)
            };

            let func: extern "thiscall" fn(*mut c_void, $($arg),+) -> $t = unsafe { mem::transmute(addr as *mut c_void) };
            func
        }
    };
}
pub(crate) use vfunc;

macro_rules! netvar {
    ($start:expr, $table:tt, $netvar:tt, $t:ty) => {
        {
            use crate::netvars;
            static mut OFFSET: usize = 0;
            if OFFSET == 0 {
                OFFSET = netvars::get($table, $netvar).unwrap();
                println!("{} @ {:#X}", $netvar, OFFSET);
            }

            *((($start as usize) + OFFSET) as *const $t)
        }
    };
}
pub(crate) use netvar;