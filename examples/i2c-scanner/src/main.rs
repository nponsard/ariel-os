#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    debug::{ExitCode, exit},
    hal,
    i2c::controller::{Kilohertz, highest_freq_in},
    log::info,
    time::Timer,
};

use embedded_hal_async::i2c::I2c;

#[ariel_os::task(autostart, peripherals)]
async fn i2c_scanner(peripherals: pins::Peripherals) {
    let mut i2c_config = hal::i2c::controller::Config::default();
    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };

    let mut i2c_bus = pins::SensorI2c::new(peripherals.i2c_sda, peripherals.i2c_scl, i2c_config);

    #[cfg(context = "nordic-thingy-91-x-nrf9151")]
    ariel_os::hal::boards::init_thingy91x_board(&mut i2c_bus, false, false)
        .await
        .unwrap();

    info!("Checking for I2C devices on the bus...");

    for addr in 1..=127 {
        if i2c_bus.write(addr, &[]).await.is_ok() {
            info!("Found device at address 0x{:x}", addr);
        }
    }
    log_value(&mut i2c_bus, 0x03, 0x04, "BCHGENABLESET").await;
    log_value(&mut i2c_bus, 0x03, 0x06, "BCHGDISABLESET").await;
    log_value(&mut i2c_bus, 0x03, 0x07, "BCHGDISABLECLR").await;
    log_value(&mut i2c_bus, 0x03, 0x08, "BCHGISETMSB").await;
    log_value(&mut i2c_bus, 0x03, 0x09, "BCHGISETLSB").await;
    // i2c_bus.write(0x6b, &[0x02, 0x09, 0x01]).await.unwrap();

    log_value(&mut i2c_bus, 0x03, 0x0A, "BCHGISETDISCHARGEMSB").await;
    log_value(&mut i2c_bus, 0x03, 0x0B, "BCHGISETDISCHARGELSB").await;
    log_value(&mut i2c_bus, 0x03, 0x0C, "BCHGVTERM").await;
    log_value(&mut i2c_bus, 0x03, 0x0D, "BCHGVTERMR").await;
    log_value(&mut i2c_bus, 0x03, 0x0E, "BCHGVTRICKLESEL").await;
    log_value(&mut i2c_bus, 0x03, 0x0F, "BCHGITERMSEL").await;
    log_value(&mut i2c_bus, 0x03, 0x3C, "BCHGCONFIG").await;
    log_value(&mut i2c_bus, 0x03, 0x50, "BCHGVBATLOWCHARGE").await;
    log_value(&mut i2c_bus, 0x03, 0x34, "BCHGCHARGESTATUS").await;

    log_value(&mut i2c_bus, 0xA0, 0x00, "LEDDRV0MODESEL").await;
    log_value(&mut i2c_bus, 0xA0, 0x01, "LEDDRV1MODESEL").await;
    log_value(&mut i2c_bus, 0xA0, 0x02, "LEDDRV2MODESEL").await;
    log_value(&mut i2c_bus, 0x02, 0x01, "VBUSINILIM0").await;
    log_value(&mut i2c_bus, 0x02, 0x02, "VBUSINILIMSTARTUP").await;
    log_value(&mut i2c_bus, 0x02, 0x03, "VBUSSUSPEND").await;

    log_value(&mut i2c_bus, 0x04, 0x00, "BUCK1ENASET").await;
    log_value(&mut i2c_bus, 0x04, 0x02, "BUCK2ENASET").await;
    log_value(&mut i2c_bus, 0x04, 0x04, "BUCK1PWMSET").await;
    log_value(&mut i2c_bus, 0x04, 0x06, "BUCK2PWMSET").await;

    log_value(&mut i2c_bus, 0x04, 0x08, "BUCK1NORMVOUT").await;
    log_value(&mut i2c_bus, 0x04, 0x09, "BUCK1RETVOUT").await;
    log_value(&mut i2c_bus, 0x04, 0x0A, "BUCK2NORMVOUT").await;
    log_value(&mut i2c_bus, 0x04, 0x0B, "BUCK2RETVOUT").await;

    log_value(&mut i2c_bus, 0x04, 0x0C, "BUCKENCTRL").await;
    log_value(&mut i2c_bus, 0x04, 0x0D, "BUCKVRETCTRL").await;
    log_value(&mut i2c_bus, 0x04, 0x0E, "BUCKPWMCTRL").await;
    log_value(&mut i2c_bus, 0x04, 0x0F, "BUCKSWCTRLSEL").await;
    log_value(&mut i2c_bus, 0x04, 0x10, "BUCK1VOUTSTATUS").await;
    log_value(&mut i2c_bus, 0x04, 0x11, "BUCK2VOUTSTATUS").await;
    log_value(&mut i2c_bus, 0x04, 0x15, "BUCKCTRL0").await;
    log_value(&mut i2c_bus, 0x04, 0x34, "BUCKSTATUS").await;

    log_value(&mut i2c_bus, 0x08, 0x04, "LDSWSTATUS").await;
    log_value(&mut i2c_bus, 0x08, 0x05, "LDSW1GPISEL").await;
    log_value(&mut i2c_bus, 0x08, 0x06, "LDSW2GPISEL").await;
    log_value(&mut i2c_bus, 0x08, 0x07, "LDSWCONFIG").await;
    log_value(&mut i2c_bus, 0x08, 0x08, "LDSW1LDOSEL").await;
    log_value(&mut i2c_bus, 0x08, 0x09, "LDSW2LDOSEL").await;
    log_value(&mut i2c_bus, 0x08, 0x0C, "LDSW1VOUTSEL").await;
    log_value(&mut i2c_bus, 0x08, 0x0D, "LDSW2VOUTSEL").await;

    info!("Done checking. Have a great day!");
    // ADCIBATMEASEN 0x05 0x24
    i2c_bus.write(0x6b, &[0x05, 0x24, 0x01]).await.unwrap();

    // EVENTSBCHARGER0CLR

    i2c_bus.write(0x6b, &[0x00, 0x07, 0xFF]).await.unwrap();

    loop {
        // TASKIBATMEASURE [0x05, 0x06]
        i2c_bus.write(0x6b, &[0x05, 0x06, 0x01]).await.unwrap();

        // TASKVBATMEASURE [0x05, 0x00]
        i2c_bus.write(0x6b, &[0x05, 0x00, 0x01]).await.unwrap();
        Timer::after_millis(1000).await;
        // ADCVBATRESULTMSB [0x05, 0x11]
        let mut buffer = [0u8, 1];

        i2c_bus
            .write_read(0x6b, &[0x05, 0x11], &mut buffer)
            .await
            .unwrap();
        let charge = (buffer[0] as u16) << 2;

        i2c_bus
            .write_read(0x6b, &[0x05, 0x15], &mut buffer)
            .await
            .unwrap();

        let charge = charge | (buffer[0] as u16 & 0x03);

        let voltage = (charge as f32 / 1023.0) * 5.0;
        info!("voltage : {}", voltage);

        // ADCIBATMEASSTATUS 0x05 0x10
        log_value(&mut i2c_bus, 0x05, 0x10, "ADCIBATMEASSTATUS").await;

        log_value(&mut i2c_bus, 0x05, 0x11, "ADCVBATRESULTMSB").await;
        log_value(&mut i2c_bus, 0x05, 0x15, "ADCGP0RESULTLSBS").await;
        log_value(&mut i2c_bus, 0x03, 0x34, "BCHGCHARGESTATUS").await;
        log_value(&mut i2c_bus, 0x03, 0x08, "BCHGISETMSB").await;
        log_value(&mut i2c_bus, 0x03, 0x09, "BCHGISETLSB").await;

        log_value(&mut i2c_bus, 0x00, 0x07, "EVENTSBCHARGER0CLR").await;

        Timer::after_millis(1000).await;
    }

    exit(ExitCode::SUCCESS);
}

async fn log_value(i2c_bus: &mut impl I2c, base: u8, offset: u8, label: &str) {
    let mut buffer = [0u8; 1];

    i2c_bus
        .write_read(0x6b, &[base, offset], &mut buffer)
        .await
        .unwrap();

    info!("{}: {:#x}", label, buffer[0]);
}
