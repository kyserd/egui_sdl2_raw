mod egui_glow;
mod egui_sdl;
mod hooks;
mod memory;
mod panic_handler;

use hooks::HOOKS;
use memory::MEMORY;
use std::{
    net::TcpStream,
    panic::{self},
    sync::Mutex,
};
use tracing::{debug, error, info, metadata::LevelFilter};

#[ctor::ctor]
fn entry() {
    // Set panic handler
    panic::set_hook(Box::new(panic_handler::panic_handler));

    // Init logging
    let stream = TcpStream::connect("127.0.0.1:7331").unwrap();
    tracing_subscriber::fmt()
        .with_writer(Mutex::new(stream))
        .with_max_level(LevelFilter::TRACE)
        .init();

    debug!("Panic hook set");
    debug!("Tracing init");

    info!("Loading SDL2...");
    info!("{:?}", MEMORY.read().unwrap());
    info!("Init SDL hooks");
    HOOKS.write().unwrap().enable();
}
