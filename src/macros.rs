macro_rules! vfunc {
    ($self:ident, $t:ty, $index:tt) => {
        {
            use std::mem;
            let addr = unsafe {
                *((*($self.start as *const usize) + mem::size_of::<usize>() * $index) as *const usize)
            };
            let func: extern "thiscall" fn(*mut c_void) -> $t = unsafe { mem::transmute(addr as *mut c_void) };
            func
        }
    };
    ($self:ident, $t:ty, $index:tt, $($arg:ty),+) => {
        {
            use std::mem;
            let addr = unsafe {
                *((*($self.start as *const usize) + mem::size_of::<usize>() * $index) as *const usize)
            };
            let func: extern "thiscall" fn(*mut c_void, $($arg),+) -> $t = unsafe { mem::transmute(addr as *mut c_void) };
            func
        }
    };
}
pub(crate) use vfunc;

macro_rules! netvar {
    ($self:ident, $table:tt, $netvar:tt, $t:ty) => {
        {
            use crate::netvars;
            static mut OFFSET: usize = 0;
            if OFFSET == 0 {
                OFFSET = netvars::get($table, $netvar).unwrap();
                println!("{} @ {:#X}", $netvar, OFFSET);
            }

            *((($self.start as usize) + OFFSET) as *const $t)
        }
    };
}
pub(crate) use netvar;