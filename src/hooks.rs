use std::{mem, ffi::{c_void, c_float}, ptr, sync::Once};

use crate::{interfaces::INTERFACES, sdk::CUserCmd};

pub unsafe fn init() {
    // Hook create_move
    println!("[Hooking]");
    // Init MH
    if minhook_sys::MH_Initialize() == minhook_sys::MH_OK {
        println!("Minhook initialized");
    } else {
        eprintln!("Minhook error");
    }

    let vtable = *(*((*(INTERFACES.client_mode as *const usize)) as *const usize) as *const usize);
    let func_addr = *((vtable + mem::size_of::<usize>() * 21) as *const usize);
    let create_move_orig = func_addr as *mut c_void;
    let mut null_orig = ptr::null_mut();
    let status = minhook_sys::MH_CreateHook(create_move_orig, hk_create_move as *mut c_void, &mut null_orig);
    if status == minhook_sys::MH_OK {
        println!("CreateMove hook created")
    } else {
        eprintln!("status: {status}");
    }

    // Enable hooks
    if minhook_sys::MH_EnableHook(ptr::null_mut()) == minhook_sys::MH_OK {
        println!("Hooks enabled");
    } else {
        eprintln!("Hooks not enabled");
    }
}

unsafe extern "stdcall" fn hk_create_move(_sampletime: c_float, _cmd: *mut CUserCmd) -> bool {
    static HOOKED: Once = Once::new();
    HOOKED.call_once(|| {
        println!("\n[CreateMove]\nGreetings from CreateMove...");
        let index = INTERFACES.engine.get_localplayer();
        println!("Localplayer index: {index}");
        let localplayer = INTERFACES.entlist.get_client_entity(index);
        if !localplayer.is_null() {
            println!("Localplayer address: {:?}", localplayer);
        }
    });
    false
}