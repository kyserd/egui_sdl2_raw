use std::{panic::PanicInfo, process::Command};
use tracing::error;

pub fn panic_handler(pi: &PanicInfo) {
    let _ = Command::new("zenity")
        .arg("--error")
        .arg(format!("--text='Atcha {}'", pi))
        .arg("--title='Panic!'")
        .spawn();
    error!("Atcha {}", pi);
}
