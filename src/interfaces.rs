use std::{ffi::{c_void, c_char, c_int, CString}, mem, ptr};

use windows::{Win32::{Foundation::HINSTANCE, System::LibraryLoader}, s, core::PCWSTR, w};

use crate::sdk::{Engine, EntityList, BaseClient};

pub struct Interfaces {
    pub client: BaseClient,
    pub client_mode: *mut c_void,
    pub engine: Engine,
    pub entlist: EntityList,
}

pub static mut INTERFACES: Interfaces = Interfaces {
    client: BaseClient{ start: 0 as *mut _ },
    client_mode: 0 as *mut _,
    engine: Engine{ start: 0 as *mut _ },
    entlist: EntityList { start: 0 as *mut _ },
};

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

pub unsafe fn init() {
    // Initialize modules
    println!("[Modules]");
    // TODO: unload on error when unwrapping
    let client_mod = get_module(w!("client.dll"), "client").unwrap();
    let engine_mod = get_module(w!("engine.dll"), "engine").unwrap();
    println!("");

    // Initialize interfaces
    println!("[Interfaces]");
    let client_factory = get_factory(client_mod).unwrap();
    let engine_factory = get_factory(engine_mod).unwrap();
    INTERFACES.client.start = get_interface(client_factory, "VClient017").unwrap();
    INTERFACES.engine.start = get_interface(engine_factory, "VEngineClient014").unwrap();
    INTERFACES.entlist.start = get_interface(client_factory, "VClientEntityList003").unwrap();

    // Get client mode
    // Read vtable address?
    let vtable = *(INTERFACES.client.start as *const usize);
    // Add pointer size * index of function
    let function_addr = vtable + mem::size_of::<usize>() * 10;
    let buf = *(function_addr as *const usize) + 5;
    INTERFACES.client_mode = buf as *mut c_void;

    if INTERFACES.client_mode.is_null() {
        eprintln!("Couldn't find client_mode interface");
    } else {
        println!("Found interface client_mode at {:?}", INTERFACES.client_mode);
    }

    println!("");
}