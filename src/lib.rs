#![feature(abi_thiscall)]
use std::{mem, thread, ffi::{c_void, c_char, c_int, CString, c_float}, time::Duration, ptr, sync::Once};
use windows::{Win32::{Foundation::HINSTANCE, System::{Console, LibraryLoader}, UI::Input::KeyboardAndMouse}, w, s, core::PCWSTR};

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

type CreateInterfaceFn = extern "C" fn(name: *const c_char, rc: *mut c_int) -> *mut c_void;
unsafe fn get_factory(module: HINSTANCE) -> Option<CreateInterfaceFn> {
    match LibraryLoader::GetProcAddress(module, s!("CreateInterface")) {
        Some(f) => Some(mem::transmute::<_, CreateInterfaceFn>(f)),
        None => None
    }
}

unsafe fn get_interface(factory: CreateInterfaceFn, version: &str) -> Option<*mut c_void> {
    let version = CString::new(version).unwrap();
    let i = factory(version.as_ptr(), ptr::null_mut());
    if i.is_null() {
        eprintln!("Couldn't get interface {version:?}");
        None
    } else {
        println!("Found interface {version:?} at {i:?}");
        Some(i)
    }
}

macro_rules! vfunctest {
    ($self:ident, $t:ty, $index:tt) => {
        {
            let addr = unsafe {
                *((*($self.start as *const usize) + mem::size_of::<usize>() * $index) as *const usize)
            };
            let func: extern "thiscall" fn(*mut c_void) -> $t = unsafe { mem::transmute(addr as *mut c_void) };
            func
        }
    };
    ($self:ident, $t:ty, $index:tt, $($arg:tt),+) => {
        {
            let addr = unsafe {
                *((*($self.start as *const usize) + mem::size_of::<usize>() * $index) as *const usize)
            };
            let func: extern "thiscall" fn(*mut c_void, $($arg),+) -> $t = unsafe { mem::transmute(addr as *mut c_void) };
            func
        }
    }
}

struct Engine {
    start: *mut c_void
}

impl Engine {
    fn get_localplayer(&self) -> i32 {
        let func = vfunctest!(self, i32, 12);
        func(self.start)
    }
}

struct EntityList {
    start: *mut c_void
}

impl EntityList {
    fn get_client_entity(&self, index: i32) -> *mut c_void {
        let func = vfunctest!(self, *mut c_void, 3, i32);
        func(self.start, index)
    }
}

struct Interfaces {
    client: *mut c_void,
    engine: Engine,
    entlist: EntityList,
}

static mut INTERFACES: Interfaces = Interfaces {
    client: 0 as *mut _,
    engine: Engine{ start: 0 as * mut _ },
    entlist: EntityList { start: 0 as * mut _ },
};

unsafe fn init(hlibmodule: HINSTANCE) {
    Console::AllocConsole();
    Console::SetConsoleTitleW(w!("Falu's TF2 Trainer"));

    // Initialize modules
    println!("[Modules]");
    let client_mod = get_module(w!("client.dll"), "client").unwrap_or_else(|| { unload(hlibmodule); unreachable!("Couldn't unload")});
    let engine_mod = get_module(w!("engine.dll"), "engine").unwrap_or_else(|| { unload(hlibmodule); unreachable!("Couldn't unload")});
    println!("");

    // Initialize interfaces
    println!("[Interfaces]");
    let client_factory = get_factory(client_mod).unwrap_or_else(|| { unload(hlibmodule); unreachable!("Couldn't unload")});
    let engine_factory = get_factory(engine_mod).unwrap_or_else(|| { unload(hlibmodule); unreachable!("Couldn't unload")});
    INTERFACES.client = get_interface(client_factory, "VClient017").unwrap_or_else(|| { unload(hlibmodule); unreachable!("Couldn't unload")});
    INTERFACES.engine.start = get_interface(engine_factory, "VEngineClient014").unwrap_or_else(|| { unload(hlibmodule); unreachable!("Couldn't unload")});
    INTERFACES.entlist.start = get_interface(client_factory, "VClientEntityList003").unwrap_or_else(|| { unload(hlibmodule); unreachable!("Couldn't unload")});
    
    // TODO: FIND SOME WAY TO CALL FUNCTION FROM ENGINES VTABLE

    // Get client mode
    // Read vtable
    let vtable = *(INTERFACES.client as *const usize);
    // Add pointer size * index of function
    let function_addr = vtable + mem::size_of::<usize>() * 10;
    let buf = *(function_addr as *const usize) + 5;
    let i_client_mode = buf as *mut c_void;

    if i_client_mode.is_null() {
        eprintln!("Couldn't find client_mode interface");
        thread::sleep(Duration::from_secs(5));
        unload(hlibmodule);
        unreachable!("Couldn't unload")
    } else {
        println!("Found interface client_mode at {:?}", i_client_mode);
    }

    println!("");

    // Hook create_move
    println!("[Hooking]");
    // Init MH
    if minhook_sys::MH_Initialize() == minhook_sys::MH_OK {
        println!("Minhook initialized");
    } else {
        eprintln!("Minhook error");
    }

    let vtable = *(*((*(i_client_mode as *const usize)) as *const usize) as *const usize);
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

    while (KeyboardAndMouse::GetAsyncKeyState(KeyboardAndMouse::VK_DELETE.0 as i32) & 1) == 0 {
        thread::sleep(Duration::from_secs(1));
    }

    unload(hlibmodule);
}

unsafe fn get_module(dll: PCWSTR, label: &str) -> Option<HINSTANCE> {
    match LibraryLoader::GetModuleHandleW(dll) {
        Ok(c) => {
            println!("Found {label} module");
            Some(c)
        },
        Err(e) => {
            eprintln!("Couldn't find {label} module: {:?}", e);
            None
        }
    }
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

struct QAngle{
    _pitch: f32,
    _yaw: f32,
    _roll: f32
}

struct CUserCmd {
    _vtable: *mut c_void,
    _command_number: i32,
    _tick_count: i32,
    _viewangles: QAngle,
    _forwardmove: f32,
    _sidemove: f32,
    _upmove: f32,
    _buttons: i32
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