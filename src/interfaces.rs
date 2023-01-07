use std::{ffi::{c_void, c_char, c_int, CString}, mem, ptr};

use windows::{Win32::{Foundation::HINSTANCE, System::LibraryLoader}, s, core::PCWSTR, w};

use crate::sdk::{Engine, EntityList, BaseClient, Surface, DebugOverlay};

pub struct Interfaces {
    pub client: BaseClient,
    pub client_mode: *mut c_void,
    pub engine: Engine,
    pub entlist: EntityList,
    pub engine_vgui: *mut c_void,
    pub surface: Surface,
    pub debug_overlay: DebugOverlay,
}

pub static mut INTERFACES: Interfaces = Interfaces {
    client: BaseClient { start: 0 as *mut _ },
    client_mode: 0 as *mut _,
    engine: Engine { start: 0 as *mut _ },
    entlist: EntityList { start: 0 as *mut _ },
    engine_vgui: 0 as *mut _,
    surface: Surface { start: 0 as *mut _ },
    debug_overlay: DebugOverlay { start: 0 as *mut _ },
};

pub unsafe fn init() {
    // Initialize modules
    println!("[Modules]");
    // TODO: unload on error when unwrapping
    let client_mod = get_module(w!("client.dll"), "client").unwrap();
    println!("client mod found at {:X}", client_mod.0);
    let engine_mod = get_module(w!("engine.dll"), "engine").unwrap();
    let matsurface_mod = get_module(w!("vguimatsurface.dll"), "vguimatsurface").unwrap();
    println!("");

    // Initialize interfaces
    println!("[Interfaces]");
    let client_factory = get_factory(client_mod).unwrap();
    let engine_factory = get_factory(engine_mod).unwrap();
    let matsurface_factory = get_factory(matsurface_mod).unwrap();
    INTERFACES.client.start = get_interface(client_factory, "VClient017").unwrap();
    INTERFACES.engine.start = get_interface(engine_factory, "VEngineClient013").unwrap();
    INTERFACES.entlist.start = get_interface(client_factory, "VClientEntityList003").unwrap();
    INTERFACES.engine_vgui = get_interface(engine_factory, "VEngineVGui002").unwrap();
    INTERFACES.surface.start = get_interface(matsurface_factory, "VGUI_Surface030").unwrap();
    INTERFACES.debug_overlay.start = get_interface(engine_factory, "VDebugOverlay003").unwrap();

    // Get client_mode
    let client_vtable = *(INTERFACES.client.start as *const usize);
    let func_addr = client_vtable + mem::size_of::<usize>() * 10;
    let final_addr = *(*((*(func_addr as *const usize) + 5) as *const usize) as *const usize);
    INTERFACES.client_mode = final_addr as *mut c_void;

    if INTERFACES.client_mode.is_null() {
        eprintln!("Couldn't find client_mode interface");
    } else {
        println!("Found interface client_mode at {:?}", INTERFACES.client_mode);
    }

    println!("");
}

type CreateInterfaceFn = extern "C" fn(name: *const c_char, rc: *mut c_int) -> *mut c_void;
unsafe fn get_factory(module: HINSTANCE) -> Option<CreateInterfaceFn> {
    match LibraryLoader::GetProcAddress(module, s!("CreateInterface")) {
        Some(f) => Some(mem::transmute::<_, CreateInterfaceFn>(f)),
        None => None
    }
}

unsafe fn get_interface(factory: CreateInterfaceFn, version: &str) -> Option<*mut c_void> {
    let c_version = CString::new(version).unwrap();
    let i = factory(c_version.as_ptr(), ptr::null_mut());
    if i.is_null() {
        eprintln!("Couldn't get interface {version:?}");
        None
    } else {
        println!("Found interface {version} at {i:?}");
        Some(i)
    }
}

pub unsafe fn get_module(dll: PCWSTR, label: &str) -> Option<HINSTANCE> {
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