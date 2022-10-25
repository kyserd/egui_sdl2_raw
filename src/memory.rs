use once_cell::sync::Lazy;
use std::os::raw::c_char;
use std::{ffi::CString, sync::RwLock};
use tracing::debug;

pub static MEMORY: Lazy<RwLock<Memory>> = Lazy::new(|| {
    let mem = Memory::new();
    RwLock::new(mem)
});

#[derive(Debug)]
pub struct Memory {
    pub poll_event: usize,
    pub swap_window: usize,
    pub get_proc_address: usize,
}

#[repr(C)]
struct Lib {
    addr: usize,
    name: *mut c_char,
}

impl Memory {
    pub fn new() -> Self {
        // Constant in case name changes
        #[allow(dead_code)]
        const LIBSDL2: &str = "libSDL2-2.0.so.0";

        // Load the library
        let c_str = CString::new(LIBSDL2).unwrap();
        let sdl = unsafe { libc::dlopen(c_str.as_ptr(), libc::RTLD_LAZY | libc::RTLD_NOLOAD) };

        // Panic on failure
        if sdl.is_null() {
            panic!("Failed to open {}", LIBSDL2);
        }

        let proc_addr_name = CString::new("SDL_GL_GetProcAddress").unwrap();
        let get_proc_address = unsafe { libc::dlsym(sdl, proc_addr_name.as_ptr()) as usize };
        // Can't be null
        if get_proc_address == 0 {
            panic!("dlsym failed to get SDL_GL_GetProcAddress");
        }

        let poll_event;
        let swap_window;
        unsafe {
            let lib = &*(sdl as *const Lib);
            let cname = CString::from_raw(lib.name);
            let name = cname.to_str().unwrap();
            debug!("{}, {}", lib.addr, name);

            // Get function table offsets
            poll_event = *(sdl as *const usize) + 0xFF /* <- your offset here */;
            swap_window = lib.addr + 0xFF /* <- your offset here */;
        }

        // Close library
        unsafe { libc::dlclose(sdl) };

        Memory {
            poll_event,
            swap_window,
            get_proc_address,
        }
    }
}
