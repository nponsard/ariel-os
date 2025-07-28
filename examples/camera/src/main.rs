#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    debug::log::info,
    hal,
    i2c::controller::{Kilohertz, highest_freq_in},
};

use embedded_hal_async::i2c::I2c;
use esp_hal::dma_rx_stream_buffer;
use esp_hal::lcd_cam::{
    LcdCam,
    cam::{Camera, Config, RxEightBits},
};
use fugit::{HertzU32, MegahertzU32};
#[ariel_os::task(autostart, peripherals)]
async fn i2c_scanner(peripherals: pins::Peripherals) {
    let mut i2c_config = hal::i2c::controller::Config::default();
    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(90)..=Kilohertz::kHz(110)) };

    let mut i2c_bus = pins::SensorI2c::new(peripherals.i2c_sda, peripherals.i2c_scl, i2c_config);

    info!("Checking for I2C devices on the bus...");

    for addr in 1..=127 {
        if i2c_bus.write(addr, &[]).await.is_ok() {
            info!("Found device at address 0x{:x}", addr);
        }
    }

    info!("Done checking. Have a great day!");
    let lcd_cam = LcdCam::new(peripherals.lcd_cam);

    let mut config = Config::default();
    config.frequency = HertzU32::MHz(40);

    let camera = esp_hal::lcd_cam::cam::Camera::new(
        lcd_cam.cam,
        peripherals.dma_ch0,
        RxEightBits::new(
            peripherals.dvp_y2,
            peripherals.dvp_y3,
            peripherals.dvp_y4,
            peripherals.dvp_y5,
            peripherals.dvp_y6,
            peripherals.dvp_y7,
            peripherals.dvp_y8,
            peripherals.dvp_y9,
        ),
        config,
    )
    .unwrap()
    .with_master_clock(peripherals.xmclk)
    .with_pixel_clock(peripherals.dvp_pclk)
    .with_ctrl_pins(peripherals.dvp_vsync, peripherals.dvp_href);
    let dma_buf = dma_rx_stream_buffer!(64_000, 2000);

    let transfer = match camera.receive(dma_buf) {
        Ok(transfer) => transfer,
        Err(e) => {
            info!("Failed to start camera transfer");
            return;
        }
    };

    let (data, ends_with_eof) = transfer.peek_until_eof();

    info!(
        "Received {} bytes, ends with EOF: {}",
        data.len(),
        ends_with_eof
    );
}
