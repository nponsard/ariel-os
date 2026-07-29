#![no_main]
#![no_std]

use ariel_os::{log::info, time::Timer};

#[ariel_os::task(autostart)]
async fn main() {
    loop {
        Timer::after_millis(500).await;

        info!("Hello World!");
    }
}
