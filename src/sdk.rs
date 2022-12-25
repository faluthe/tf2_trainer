use std::ffi::c_void;
use crate::macros::vfunc;

// Classes
pub struct Engine {
    pub start: *mut c_void
}

pub struct EntityList {
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

// Structs
struct QAngle{
    _pitch: f32,
    _yaw: f32,
    _roll: f32
}

pub struct CUserCmd {
    _vtable: *mut c_void,
    _command_number: i32,
    _tick_count: i32,
    _viewangles: QAngle,
    _forwardmove: f32,
    _sidemove: f32,
    _upmove: f32,
    _buttons: i32
}