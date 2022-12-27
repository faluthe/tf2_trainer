use std::ffi::{c_void, c_int, c_char};
use crate::{macros::vfunc, netvars};

// Classes
pub struct Engine {
    pub start: *mut c_void
}

pub struct EntityList {
    pub start: *mut c_void
}

pub struct BaseClient {
    pub start: *mut c_void
}

pub struct PlayerEntity {
    pub start: *mut c_void
}

// Impls
impl Engine {
    pub fn get_localplayer(&self) -> i32 {
        let func = vfunc!(self, i32, 12);
        func(self.start)
    }
}

impl EntityList {
    pub fn get_client_entity(&self, index: i32) -> *mut c_void {
        let func = vfunc!(self, *mut c_void, 3, i32);
        func(self.start, index)
    }
}

impl BaseClient {
    pub fn get_all_classes(&self) -> *mut ClientClass {
        let func = vfunc!(self, *mut ClientClass, 8);
        func(self.start)
    }
}

impl PlayerEntity {
    pub unsafe fn health(&self) -> i32 {
        // netvar!(self, "DT_CSPlayer", "m_fFlags", i32)
        static mut OFFSET: usize = 0;
        if OFFSET == 0 {
            OFFSET = netvars::get("DT_BasePlayer", "m_iHealth").unwrap();
        }

        *(((self.start as usize) + OFFSET) as *const i32)
    }
    pub unsafe fn flags(&self) -> i32 {
        static mut OFFSET: usize = 0;
        if OFFSET == 0 {
            OFFSET = netvars::get("DT_BasePlayer", "m_fFlags").unwrap();
        }

        *(((self.start as usize) + OFFSET) as *const i32)
    }
}

// Structs
struct QAngle{
    _pitch: f32,
    _yaw: f32,
    _roll: f32,
}

pub struct CUserCmd {
    _vtable: *mut c_void,
    _command_number: i32,
    _tick_count: i32,
    _viewangles: QAngle,
    _forwardmove: f32,
    _sidemove: f32,
    _upmove: f32,
    pub buttons: i32,
}

pub struct ClientClass {
    _pad: [u8; 8],
    _network_name: *const c_char,
    pub recv_table: *mut RecvTable,
    pub next: *mut ClientClass,
    _id: c_int,
}

pub struct RecvTable {
    pub props: *mut RecvProp,
    pub nprops: c_int,
    _decoder: *mut c_void,
    pub net_table_name: *mut c_char,
}

// Needs to be of size 3C!! netvars::check_table uses .offset()
pub struct RecvProp {
    pub var_name: *mut c_char,
    _pad0: [u8; 4],
    _flags: c_int,
    _str_buf_size: c_int,
    _b_inside_array: c_int,
    _extra: *const c_void,
    _array_prop: *mut RecvProp,
    _pad1: [u8; 12],
    pub data_table: *mut RecvTable,
    pub offset: c_int,
    _elementstride: c_int,
    _n_elements: c_int,
    _parent_array_prop_name: *const c_char,
}