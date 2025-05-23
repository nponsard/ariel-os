use embassy_nrf::bind_interrupts;

bind_interrupts!(struct Irqs2 {
        CLOCK_POWER => nrf_sdc::mpsl::ClockInterruptHandler;
});
