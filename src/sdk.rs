use std::ffi::{c_void, c_int, c_char, c_ulong, CString};

use windows::core::PCWSTR;

use crate::{macros::{vfunc, netvar}, interfaces::INTERFACES};

// Interfaces
pub struct Engine { pub start: *mut c_void }
pub struct EntityList { pub start: *mut c_void }
pub struct BaseClient { pub start: *mut c_void }
pub struct Surface { pub start: *mut c_void }

// Classes
pub struct PlayerEntity { pub start: *mut c_void }
pub struct WeaponEntity { pub start: *mut c_void }

// Interface impls
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

impl Surface {
    pub fn text_create_font(&self) -> c_ulong {
        let func = vfunc!(self, c_ulong, 66);
        func(self.start)
    }
    pub fn set_font_glyph_set(&self, font: c_ulong, font_name: &str, size: c_int, weight: c_int, flags: c_int) -> bool {
        let func = vfunc!(self, bool, 67, c_ulong, *const c_char, c_int, c_int, c_int, c_int, c_int, c_int, c_int);
        let font_name = CString::new(font_name).unwrap();
        func(self.start, font, font_name.as_ptr(), size, weight, 0, 0, flags, 0, 0)
    }
    pub fn draw_set_text_font(&self, font: c_ulong) {
        let func = vfunc!(self, (), 17, c_ulong);
        func(self.start, font)
    }
    pub fn draw_set_text_color(&self, r: c_int, g: c_int, b: c_int, a: c_int) {
        let func = vfunc!(self, (), 19, c_int, c_int, c_int, c_int);
        func(self.start, r, g, b, a)
    }
    pub fn draw_set_text_pos(&self, x: c_int, y: c_int) {
        let func = vfunc!(self, (), 20, c_int, c_int);
        func(self.start, x, y)
    }
    pub fn draw_print_text(&self, text: PCWSTR, len: c_int) {
        let func = vfunc!(self, (), 22, PCWSTR, c_int, c_int);
        func(self.start, text, len, 0)
    }
}

// Class impls
impl PlayerEntity {
    pub unsafe fn _health(&self) -> i32 {
        netvar!(self, "DT_BasePlayer", "m_iHealth", i32)
    }
    pub unsafe fn flags(&self) -> i32 {
        netvar!(self, "DT_BasePlayer", "m_fFlags", i32)
    }
    pub unsafe fn active_weapon(&self) -> Option<WeaponEntity> {
        let weapon = netvar!(self, "DT_BaseCombatCharacter", "m_hActiveWeapon", usize);
        let start = INTERFACES.entlist.get_client_entity(weapon as i32 & 0xFFF);
        if !start.is_null() {
            Some(WeaponEntity { start })
        } else {
            None
        }
    }
}

impl WeaponEntity {
    pub unsafe fn can_backstab(&self) -> bool {
        netvar!(self, "DT_TFWeaponKnife", "m_bReadyToBackstab", bool)
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