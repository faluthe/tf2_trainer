use std::{ffi::{c_void, c_ulong}, mem, ptr, sync::Once};

use windows::w;

use crate::{esp, interfaces::INTERFACES, sdk::CUserCmd, helpers};

// Original functions
static mut O_CREATEMOVE: *mut c_void = ptr::null_mut();
static mut O_PAINT: *mut c_void = ptr::null_mut();

pub unsafe fn init() {
    if minhook_sys::MH_Initialize() == minhook_sys::MH_OK {
        println!("Minhook initialized");
    } else { eprintln!("Minhook error"); }

    O_CREATEMOVE = create_hook(INTERFACES.client_mode.start, 21, hk_create_move as *mut c_void);
    println!("Created CreateMove hook");
    O_PAINT = create_hook(INTERFACES.engine_vgui.start, 14, hk_paint as *mut c_void);
    println!("Created Paint hook");

    // Enable hooks
    println!("Enabling hooks...\n");
    if minhook_sys::MH_EnableHook(ptr::null_mut()) != minhook_sys::MH_OK {
        eprintln!("Hooks NOT enabled!");
    } else { println!("Hooks enabled"); }
}

unsafe fn create_hook(iface: *mut c_void, index: usize, hk_func: *mut c_void) -> *mut c_void {
    let vtable = *(iface as *const usize);
    let func_addr = *((vtable + (mem::size_of::<usize>() * index)) as *const usize);
    
    let mut original = ptr::null_mut();
    minhook_sys::MH_CreateHook(func_addr as *mut c_void, hk_func, &mut original);
    
    original
}

unsafe extern "stdcall" fn hk_create_move(sample_time: f32, cmd: *mut CUserCmd) -> bool {
    let original: extern "stdcall" fn(f32, *mut CUserCmd) = mem::transmute(O_CREATEMOVE);
    original(sample_time, cmd);

    let localplayer = match helpers::localplayer() {
        Some(p) => p,
        None => return false,
    };

    if (localplayer.flags() & 1) == 0 {
        (*cmd).buttons &= !2;
    }
    
    if let Some(weapon) = localplayer.active_weapon() {
        if weapon.is_knife() && weapon.can_backstab() {
            (*cmd).buttons |= 1;
        }
    }

    false
}

unsafe extern "fastcall" fn hk_paint(ecx: usize, edx: usize, mode: i32) {
    let original: extern "fastcall" fn(usize, usize, i32) = mem::transmute(O_PAINT);
    original(ecx, edx, mode);

    static HOOKED: Once = Once::new();
    static mut FONT: c_ulong = 0;
    HOOKED.call_once(|| {
        println!("Paint hooked!");
        FONT = INTERFACES.surface.text_create_font();
        INTERFACES.surface.set_font_glyph_set(FONT, "Courier New", 15, 300, 0x210);
        println!("Font initialized!");
    });

    INTERFACES.surface.draw_set_text_font(FONT);
    INTERFACES.surface.draw_set_text_color(255, 255, 255, 255);
    INTERFACES.surface.draw_set_text_pos(10, 10);
    INTERFACES.surface.draw_print_text(w!("Falu's TF2 Trainer"), 18);

    esp::player_esp(FONT);
}