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
    ($self:ident, $t:ty, $index:tt, $($arg:tt),+) => {
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

// TODO: A macro that unloads on error