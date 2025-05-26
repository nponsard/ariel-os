use crate::irqs::Irqs;
use ariel_os_debug::log::debug;
use embassy_executor::Spawner;
use embassy_nrf::peripherals;
use embassy_nrf::peripherals::RNG;
use embassy_nrf::rng;
use embassy_sync::once_lock::OnceLock;
use nrf_sdc::SoftdeviceController;
use nrf_sdc::mpsl::MultiprotocolServiceLayer;
use nrf_sdc::{self as sdc, mpsl};
use static_cell::StaticCell;
use trouble_host::prelude::*;

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {

    debug!("Starting MPSL task");
    mpsl.run().await
}

pub static STACK: OnceLock<Stack<'static, SoftdeviceController<'static>>> = OnceLock::new();

pub async fn ble_stack() -> Host<'static, SoftdeviceController<'static>> {
    STACK.get().await.build()
}

pub struct Peripherals {
    pub ppi_ch17: peripherals::PPI_CH17,
    pub ppi_ch18: peripherals::PPI_CH18,
    pub ppi_ch20: peripherals::PPI_CH20,
    pub ppi_ch21: peripherals::PPI_CH21,
    pub ppi_ch22: peripherals::PPI_CH22,
    pub ppi_ch23: peripherals::PPI_CH23,
    pub ppi_ch24: peripherals::PPI_CH24,
    pub ppi_ch25: peripherals::PPI_CH25,
    pub ppi_ch26: peripherals::PPI_CH26,
    pub ppi_ch27: peripherals::PPI_CH27,
    pub ppi_ch28: peripherals::PPI_CH28,
    pub ppi_ch29: peripherals::PPI_CH29,

    pub rtc0: peripherals::RTC0,
    pub timer0: peripherals::TIMER0,
    pub temp: peripherals::TEMP,
    pub ppi_ch19: peripherals::PPI_CH19,
    pub ppi_ch30: peripherals::PPI_CH30,
    pub ppi_ch31: peripherals::PPI_CH31,

    pub rng: peripherals::RNG,
}

impl Peripherals {
    #[must_use]
    pub fn new(peripherals: &mut crate::OptionalPeripherals) -> Self {
        Self {
            ppi_ch17: peripherals.PPI_CH17.take().unwrap(),
            ppi_ch18: peripherals.PPI_CH18.take().unwrap(),
            ppi_ch20: peripherals.PPI_CH20.take().unwrap(),
            ppi_ch21: peripherals.PPI_CH21.take().unwrap(),
            ppi_ch22: peripherals.PPI_CH22.take().unwrap(),
            ppi_ch23: peripherals.PPI_CH23.take().unwrap(),
            ppi_ch24: peripherals.PPI_CH24.take().unwrap(),
            ppi_ch25: peripherals.PPI_CH25.take().unwrap(),
            ppi_ch26: peripherals.PPI_CH26.take().unwrap(),
            ppi_ch27: peripherals.PPI_CH27.take().unwrap(),
            ppi_ch28: peripherals.PPI_CH28.take().unwrap(),
            ppi_ch29: peripherals.PPI_CH29.take().unwrap(),

            rtc0: peripherals.RTC0.take().unwrap(),
            timer0: peripherals.TIMER0.take().unwrap(),
            temp: peripherals.TEMP.take().unwrap(),
            ppi_ch19: peripherals.PPI_CH19.take().unwrap(),
            ppi_ch30: peripherals.PPI_CH30.take().unwrap(),
            ppi_ch31: peripherals.PPI_CH31.take().unwrap(),

            rng: peripherals.RNG.take().unwrap(),
        }
    }
}

fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut rng::Rng<RNG>,
    mpsl: &'d MultiprotocolServiceLayer,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    sdc::Builder::new().unwrap()
        .support_adv().unwrap()
        // .support_peripheral().unwrap()
        // .peripheral_count(1).unwrap()
        .build(p, rng, mpsl, mem)
}

pub fn driver<'d>(
    p: Peripherals,
    spawner: &Spawner,
) -> Stack<'static, SoftdeviceController<'static>> {
    let mpsl_p =
        mpsl::Peripherals::new(p.rtc0, p.timer0, p.temp, p.ppi_ch19, p.ppi_ch30, p.ppi_ch31);
    let lfclk_cfg = mpsl::raw::mpsl_clock_lfclk_cfg_t {
        source: mpsl::raw::MPSL_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_CTIV as u8,
        rc_temp_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_TEMP_CTIV as u8,
        accuracy_ppm: mpsl::raw::MPSL_DEFAULT_CLOCK_ACCURACY_PPM as u16,
        skip_wait_lfclk_started: mpsl::raw::MPSL_DEFAULT_SKIP_WAIT_LFCLK_STARTED != 0,
    };
    static MPSL: StaticCell<MultiprotocolServiceLayer> = StaticCell::new();
    let mpsl = MPSL.init(mpsl::MultiprotocolServiceLayer::new(mpsl_p, Irqs, lfclk_cfg).unwrap());
    spawner.must_spawn(mpsl_task(&*mpsl));

    let sdc_p = sdc::Peripherals::new(
        p.ppi_ch17, p.ppi_ch18, p.ppi_ch20, p.ppi_ch21, p.ppi_ch22, p.ppi_ch23, p.ppi_ch24,
        p.ppi_ch25, p.ppi_ch26, p.ppi_ch27, p.ppi_ch28, p.ppi_ch29,
    );

    static RNG: StaticCell<embassy_nrf::rng::Rng<'static, RNG>> = StaticCell::new();

    let rng = RNG.init(embassy_nrf::rng::Rng::new(p.rng, Irqs));

    static SDC_MEM: StaticCell<sdc::Mem<1024>> = StaticCell::new();

    let sdc_mem = SDC_MEM.init(sdc::Mem::<1024>::new());

    let sdc = build_sdc(sdc_p, rng, mpsl, sdc_mem).unwrap();

    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);

    debug!("BLE address set");

    static HOST_RESOURCES: StaticCell<HostResources<1, 1, 27>> = StaticCell::new();

    let resources = HOST_RESOURCES.init(HostResources::new());

    static STACK_BUILDER: StaticCell<Stack<'static, SoftdeviceController<'static>>> =
        StaticCell::new();

    debug!("creating stack");
    trouble_host::new(sdc, resources).set_random_address(address)
}
