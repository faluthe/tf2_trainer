use crate::sdk::{BaseClient, DebugOverlay, Engine, EntityList, Surface};
use lazy_static::lazy_static;
use std::{ffi::{c_void, c_char, CString}, mem, ptr};
use windows::{Win32::{Foundation::HINSTANCE, System::LibraryLoader}, s, w};

pub struct DefaultInterface { pub start: *mut c_void }
unsafe impl Send for DefaultInterface {}
unsafe impl Sync for DefaultInterface {}
pub struct Interfaces {
    pub client: BaseClient,
    pub client_mode: DefaultInterface,
    pub debug_overlay: DebugOverlay,
    pub engine: Engine,
    pub engine_vgui: DefaultInterface,
    pub entlist: EntityList,
    pub surface: Surface,
}

// TODO: need more error checking here (client_mode not checked, wait until all modules are loaded)
lazy_static! { pub static ref INTERFACES: Interfaces = { unsafe {
    // TODO: unload on error when unwrapping
    let client_mod = LibraryLoader::GetModuleHandleW(w!("client.dll")).unwrap();
    println!("client.dll @ {:X}", client_mod.0);
    let engine_mod = LibraryLoader::GetModuleHandleW(w!("engine.dll")).unwrap();
    println!("engine.dll @ {:X}", engine_mod.0);
    let matsurface_mod = LibraryLoader::GetModuleHandleW(w!("vguimatsurface.dll")).unwrap();
    println!("vguimatsurface.dll @ {:X}", matsurface_mod.0);

    let client_factory = get_factory(client_mod).unwrap();
    let engine_factory = get_factory(engine_mod).unwrap();
    let matsurface_factory = get_factory(matsurface_mod).unwrap();

    let client = BaseClient { start: get_interface(client_factory, "VClient017").unwrap() };
    let debug_overlay = DebugOverlay{ start: get_interface(engine_factory, "VDebugOverlay003").unwrap() };
    let engine = Engine { start: get_interface(engine_factory, "VEngineClient013").unwrap() };
    let entlist = EntityList { start: get_interface(client_factory, "VClientEntityList003").unwrap() };
    let engine_vgui = DefaultInterface{ start: get_interface(engine_factory, "VEngineVGui002").unwrap() };
    let surface = Surface { start: get_interface(matsurface_factory, "VGUI_Surface030").unwrap() };

    let client_mode = DefaultInterface { start: {
        let client_vtable = *(INTERFACES.client.start as *const usize);
        let func_addr = client_vtable + mem::size_of::<usize>() * 10;
        let final_addr = *(*((*(func_addr as *const usize) + 5) as *const usize) as *const usize);
        final_addr as *mut c_void
    }};

    Interfaces { client, client_mode, debug_overlay, engine, engine_vgui, entlist, surface }
}};}

type CreateInterfaceFn = extern "C" fn(name: *const c_char, rc: *mut i32) -> *mut c_void;
unsafe fn get_factory(module: HINSTANCE) -> Option<CreateInterfaceFn> {
    match LibraryLoader::GetProcAddress(module, s!("CreateInterface")) {
        Some(f) => Some(mem::transmute::<_, CreateInterfaceFn>(f)),
        None => None
    }
}

fn get_interface(factory: CreateInterfaceFn, version: &str) -> Option<*mut c_void> {
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