use ariel_os::hal::{i2c, peripherals};

#[cfg(context = "xiao-esp32-s3-sense")]
pub type SensorI2c = i2c::controller::I2C0;
#[cfg(context = "xiao-esp32-s3-sense")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: GPIO40,
    i2c_scl: GPIO39,

    xmclk: GPIO10,

    dvp_vsync: GPIO38,
    dvp_pclk: GPIO13,
    dvp_href: GPIO47,

    dvp_y2: GPIO15,
    dvp_y3: GPIO17,
    dvp_y4: GPIO18,
    dvp_y5: GPIO16,
    dvp_y6: GPIO14,
    dvp_y7: GPIO12,
    dvp_y8: GPIO11,
    dvp_y9: GPIO48,

    user_led: GPIO21,

    dma_ch0: DMA_CH0,
    lcd_cam: LCD_CAM,
});
