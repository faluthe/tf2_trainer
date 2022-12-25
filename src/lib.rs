#![feature(abi_thiscall)]
use std::{thread, ffi::c_void, time::Duration, ptr};
use windows::{Win32::{Foundation::HINSTANCE, System::{Console, LibraryLoader}, UI::Input::KeyboardAndMouse}, w};

mod macros;
mod interfaces;
mod sdk;
mod hooks;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn DllMain(
    hlibmodule: HINSTANCE,
    ul_reason_for_call: u32,
    _lpreserved: *mut c_void
) -> bool {
    // DLL_PROCESS_ATTACH
    if ul_reason_for_call == 1 {
        LibraryLoader::DisableThreadLibraryCalls(hlibmodule);
        thread::spawn(move || init(hlibmodule));
    }

    true
}

unsafe fn init(hlibmodule: HINSTANCE) {
    Console::AllocConsole();
    Console::SetConsoleTitleW(w!("Falu's TF2 Trainer"));

    interfaces::init();
    hooks::init();

    while (KeyboardAndMouse::GetAsyncKeyState(KeyboardAndMouse::VK_DELETE.0 as i32) & 1) == 0 {
        thread::sleep(Duration::from_secs(1));
    }

    unload(hlibmodule);
}

unsafe fn unload(hlibmodule: HINSTANCE) {
    if minhook_sys::MH_DisableHook(ptr::null_mut()) != minhook_sys::MH_OK {
        eprintln!("Couldn't disable hooks");
        thread::sleep(Duration::from_secs(5));
    }

    if minhook_sys::MH_Uninitialize() != minhook_sys::MH_OK {
        eprintln!("Couldn't uninitialize minhook");
        thread::sleep(Duration::from_secs(5));
    }

    Console::FreeConsole();
    LibraryLoader::FreeLibraryAndExitThread(hlibmodule, 0);
}