use crate::egui_glow::EguiGlow;
use crate::egui_sdl::Platform;
use crate::memory::{Memory, MEMORY};

use libc::{c_char, c_int, c_void};
use once_cell::sync::Lazy;
use sdl2_sys::{SDL_Event, SDL_Window};
use std::ffi::CString;
use std::sync::{Arc, RwLock};
use tracing::{debug, info, trace};

#[allow(dead_code)]
type PollEventFn = unsafe extern "C" fn(event: *mut SDL_Event) -> c_int;
#[allow(dead_code)]
type SwapWindowFn = unsafe extern "C" fn(window: *mut SDL_Window);
#[allow(dead_code)]
type GetProcAddressFn = unsafe extern "C" fn(proc: *const c_char) -> *mut c_void;

pub static HOOKS: Lazy<RwLock<Hooks>> = Lazy::new(|| {
    let hooks = Hooks::new(&MEMORY.read().unwrap());
    RwLock::new(hooks)
});

pub struct Hooks {
    poll_event: PollEventFn,
    swap_window: SwapWindowFn,
}

impl Hooks {
    pub fn new(mem: &Memory) -> Self {
        let poll_event = unsafe { *(mem.poll_event as *const PollEventFn) };
        let swap_window = unsafe { *(mem.swap_window as *const SwapWindowFn) };

        debug!("{:#x}, {:#x}", poll_event as usize, swap_window as usize);
        Hooks {
            poll_event,
            swap_window,
        }
    }

    pub fn enable(&mut self) {
        unsafe {
            // PollEvent hook
            let poll_event_ptr = MEMORY.read().unwrap().poll_event as *mut usize;
            debug!(
                "PollEvent: ({:#x}, {:#x})",
                poll_event_ptr as usize, *poll_event_ptr
            );

            poll_event_ptr.write(poll_event as usize);
            debug!("PollEvent: {:#x}", *poll_event_ptr);

            // SwapWindow hook
            let swap_window_ptr = MEMORY.read().unwrap().swap_window as *mut usize;
            debug!(
                "SwapWindow: ({:#x}, {:#x})",
                swap_window_ptr as usize, *swap_window_ptr
            );
            swap_window_ptr.write(swap_window as usize);
            debug!("SwapWindow: {:#x}", *swap_window_ptr);
        }
        info!("Hooks enabled!")
    }
}

static RENDERER: Lazy<RwLock<EguiGlow>> = Lazy::new(|| {
    RwLock::new(EguiGlow::new(
        Arc::new(unsafe {
            glow::Context::from_loader_function(|name: &str| -> *const libc::c_void {
                let name = CString::new(name).unwrap();
                let func = std::mem::transmute::<usize, GetProcAddressFn>(
                    MEMORY.read().unwrap().get_proc_address,
                );
                func(name.as_ptr())
            })
        }),
        (1280, 720),
    ))
});

unsafe extern "C" fn poll_event(event: *mut SDL_Event) -> c_int {
    let result = (HOOKS.read().unwrap().poll_event)(event);
    if result != 0 {
        let platform = Platform::handle_event(event);
        let mut renderer = RENDERER.write().unwrap();
        renderer.set_raw_input(platform);
    }
    result
}

unsafe extern "C" fn swap_window(window: *mut SDL_Window) {
    let mut renderer = RENDERER.write().unwrap();

    renderer.run(|egui_ctx| {
        egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
            ui.heading("Hello World!");
            if ui.button("Log").clicked() {
                debug!("Button Clicked");
            }
        });
        egui::Window::new("Atcha").show(egui_ctx, |ui| ui.label("egui + glow + sdl"));
    });

    renderer.paint([1280, 720]);

    return (HOOKS.read().unwrap().swap_window)(window);
}
