#![no_main]
#![no_std]

use ariel_os::{
    asynch::blocker::block_on,
    debug::{ExitCode, exit, log::info},
    time::Timer,
};

#[ariel_os::thread(autostart)]
fn main() {
    info!("Hello from main thread!");

    for i in 0..10 {
        block_on(Timer::after_millis(100));
        info!("Hello again #{}", i);
    }

    exit(ExitCode::SUCCESS);
}
