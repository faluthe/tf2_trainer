use std::{mem, ffi::{c_void, c_float, c_ulong}, ptr, sync::Once};

use windows::w;

use crate::{interfaces::INTERFACES, sdk::{CUserCmd, PlayerEntity}, esp};

// Originals
static mut PAINT: *mut c_void = ptr::null_mut();

pub unsafe fn init() {
    // Hook create_move
    println!("[Hooking]");
    // Init MH
    if minhook_sys::MH_Initialize() == minhook_sys::MH_OK {
        println!("Minhook initialized");
    } else {
        eprintln!("Minhook error");
    }

    create_hook(INTERFACES.client_mode, 21, hk_create_move as *mut c_void);
    println!("Created CreateMove hook");
    PAINT = create_hook(INTERFACES.engine_vgui, 14, hk_paint as *mut c_void);
    println!("Created Paint hook");

    // Enable hooks
    println!("Enabling hooks...\n");
    if minhook_sys::MH_EnableHook(ptr::null_mut()) != minhook_sys::MH_OK {
        eprintln!("Hooks NOT enabled!");
    }
}

unsafe fn create_hook(iface: *mut c_void, index: usize, hk_func: *mut c_void) -> *mut c_void {
    let vtable = *(iface as *const usize);
    let func_addr = *((vtable + (mem::size_of::<usize>() * index)) as *const usize);
    let mut original = ptr::null_mut();
    
    minhook_sys::MH_CreateHook(func_addr as *mut c_void, hk_func, &mut original);
    
    original
}

unsafe extern "stdcall" fn hk_create_move(_sampletime: c_float, cmd: *mut CUserCmd) -> bool {
    let index = INTERFACES.engine.get_localplayer();
    let plocal = INTERFACES.entlist.get_client_entity(index);
    let localplayer = PlayerEntity{ start: plocal };

    static HOOKED: Once = Once::new();
    HOOKED.call_once(|| {
        println!("CreateMove hooked!");
        if !plocal.is_null() {
            println!("Localplayer address: {:?}", plocal);
        }
    });

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
    let original: extern "fastcall" fn(usize, usize, i32) = mem::transmute(PAINT);
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