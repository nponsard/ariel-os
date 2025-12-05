#![no_main]
#![no_std]

use ariel_os::debug as ariel_os_debug;

use ariel_os::debug::{ExitCode, exit, log::*};

#[ariel_os::thread(autostart)]
fn main() {
    info!("Hello World!");

    exit(ExitCode::SUCCESS);
}
