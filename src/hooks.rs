use std::{mem, ffi::{c_void, c_float}, ptr, sync::Once};

use crate::{interfaces::INTERFACES, sdk::{CUserCmd, PlayerEntity}};

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

    println!("");
}

static HOOKED: Once = Once::new();
unsafe extern "stdcall" fn hk_create_move(_sampletime: c_float, cmd: *mut CUserCmd) -> bool {
    HOOKED.call_once(|| {
        println!("[CreateMove]");
        let index = INTERFACES.engine.get_localplayer();
        let plocal = INTERFACES.entlist.get_client_entity(index);
        if !plocal.is_null() {
            println!("Localplayer address: {:?}", plocal);
        }
        let localplayer = PlayerEntity{ start: plocal };
        println!("Health: {}", localplayer.health());
    });

    let index = INTERFACES.engine.get_localplayer();
    let plocal = INTERFACES.entlist.get_client_entity(index);
    let localplayer = PlayerEntity{ start: plocal };

    if (localplayer.flags() & 1) == 0 {
        (*cmd).buttons &= !2;
    }

    false
}