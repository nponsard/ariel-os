use ariel_os::hal::peripherals;

#[cfg(context = "numworks")]
ariel_os::hal::define_peripherals!(LcdPeripherals {
    fsmc_d4: PA2,
    fsmc_d5: PA3,
    fsmc_d6: PA4,
    fsmc_d13: PB12,
    fsmc_d2: PD0,
    fsmc_d3: PD1,
    fsmc_noe: PD4,
    fsmc_nwe: PD5,
    fsmc_ne1: PD7,
    fsmc_d14: PD9,
    fsmc_d15: PD10,
    fsmc_a16: PD11,
    fsmc_d0: PD14,
    fsmc_d1: PD15,
    fsmc_d7: PE10,
    fsmc_d8: PE11,
    fsmc_d9: PE12,
    fsmc_d10: PE13,
    fsmc_d11: PE14,
    fsmc_d12: PE15,

    lcd_light: PC6,
    lcd_pow_en: PB14,
    lcd_reset: PE9,
    lcd_ext: PB13,

    lcd_csx: PC4,


    led_blue: PB0,
});
