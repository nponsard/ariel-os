
use crate::irqs::Irqs;
use ariel_os_debug::log::{debug, info};
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf::peripherals;
use embassy_nrf::peripherals::RNG;
use embassy_nrf::rng;
use embassy_sync::once_lock::OnceLock;
use embassy_time::{Duration, Timer};
use nrf_sdc::SoftdeviceController;
use nrf_sdc::mpsl::MultiprotocolServiceLayer;
use nrf_sdc::{self as sdc, mpsl};
use static_cell::StaticCell;
use trouble_host::prelude::*;

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
    info!("Waiting for MPSL to start aaaaaaaaaaaaaaaaaaaa");
    mpsl.run().await;

    info!("MPSL task finished");
}
const L2CAP_CHANNELS_MAX: usize = 3;
static RNG_INSTANCE: StaticCell<embassy_nrf::rng::Rng<'static, RNG>> = StaticCell::new();
static STACK_BUILDER: StaticCell<Stack<'static, SoftdeviceController<'static>>> = StaticCell::new();
static HOST_RESOURCES: StaticCell<HostResources<1, 1, L2CAP_MTU>> = StaticCell::new();
static SDC_MEM: StaticCell<sdc::Mem<8192>> = StaticCell::new();

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
const CONNECTIONS_MAX: usize = 1;
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

/// How many outgoing L2CAP buffers per link
const L2CAP_TXQ: u8 = 20;

/// How many incoming L2CAP buffers per link
const L2CAP_RXQ: u8 = 20;

const L2CAP_MTU: usize = 27;

fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut rng::Rng<RNG>,
    mpsl: &'d MultiprotocolServiceLayer,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    sdc::Builder::new()
        .unwrap()
        .support_adv()
        .unwrap()
        .support_peripheral()
        .unwrap()
        .peripheral_count(1)
        .unwrap()
        .buffer_cfg(L2CAP_MTU as u8, L2CAP_MTU as u8, L2CAP_TXQ, L2CAP_RXQ)
        .unwrap()
        .build(p, rng, mpsl, mem)
}

static mut PERIPHERALS: Option<Peripherals> = None;

pub fn driver<'d>(p: Peripherals) {
    unsafe {
        PERIPHERALS = Some(p);
    }
}
#[allow(static_mut_refs)]
pub async fn run_example<'d>(spawner: Spawner, p: &mut crate::OptionalPeripherals) {
    // let p = unsafe { PERIPHERALS.take().unwrap() };

    let spawner = Spawner::for_current_executor().await;

    let mpsl_p =
        // mpsl::Peripherals::new(p.rtc0, p.timer0, p.temp, p.ppi_ch19, p.ppi_ch30, p.ppi_ch31);

    mpsl::Peripherals::new(p.RTC0.take().unwrap(), p.TIMER0.take().unwrap(), p.TEMP.take().unwrap(), p.PPI_CH19.take().unwrap(), p.PPI_CH30.take().unwrap(), p.PPI_CH31.take().unwrap());
    let lfclk_cfg = mpsl::raw::mpsl_clock_lfclk_cfg_t {
        source: mpsl::raw::MPSL_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_CTIV as u8,
        rc_temp_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_TEMP_CTIV as u8,
        accuracy_ppm: mpsl::raw::MPSL_DEFAULT_CLOCK_ACCURACY_PPM as u16,
        skip_wait_lfclk_started: mpsl::raw::MPSL_DEFAULT_SKIP_WAIT_LFCLK_STARTED != 0,
    };
    info!("starting MPSL");

    static MPSL: StaticCell<MultiprotocolServiceLayer> = StaticCell::new();
    let mpsl = MPSL.init(mpsl::MultiprotocolServiceLayer::new(mpsl_p, Irqs, lfclk_cfg).unwrap());
    spawner.must_spawn(mpsl_task(&*mpsl));
    info!("waiting for MPSL");
    Timer::after(Duration::from_secs(1)).await;

    info!("contiuing with SDC setup");

    // let sdc_p = sdc::Peripherals::new(
    //     p.ppi_ch17, p.ppi_ch18, p.ppi_ch20, p.ppi_ch21, p.ppi_ch22, p.ppi_ch23, p.ppi_ch24,
    //     p.ppi_ch25, p.ppi_ch26, p.ppi_ch27, p.ppi_ch28, p.ppi_ch29,
    // );
    let sdc_p = sdc::Peripherals::new(
        p.PPI_CH17.take().unwrap(),
        p.PPI_CH18.take().unwrap(),
        p.PPI_CH20.take().unwrap(),
        p.PPI_CH21.take().unwrap(),
        p.PPI_CH22.take().unwrap(),
        p.PPI_CH23.take().unwrap(),
        p.PPI_CH24.take().unwrap(),
        p.PPI_CH25.take().unwrap(),
        p.PPI_CH26.take().unwrap(),
        p.PPI_CH27.take().unwrap(),
        p.PPI_CH28.take().unwrap(),
        p.PPI_CH29.take().unwrap(),
    );

    // let mut rng = embassy_nrf::rng::Rng::new(p.rng, Irqs);
    let mut rng = rng::Rng::new(p.RNG.take().unwrap(), Irqs);

    let mut sdc_mem = sdc::Mem::<12848>::new();

    info!("building SDC");

    let sdc = build_sdc(sdc_p, &mut rng, mpsl, &mut sdc_mem).unwrap();

    run::<_, L2CAP_MTU>(sdc).await;
}

pub async fn run<C, const L2CAP_MTU: usize>(controller: C)
where
    C: Controller,
{
    // Hardcoded peripheral address
    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    info!("Our address has been set ");

    let mut resources: HostResources<CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU> =
        HostResources::new();
    let stack = trouble_host::new(controller, &mut resources).set_random_address(address);
    let Host {
        mut peripheral,
        mut runner,
        ..
    } = stack.build();

    let mut adv_data = [0; 31];
    AdStructure::encode_slice(
        &[AdStructure::Flags(
            LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED,
        )],
        &mut adv_data[..],
    )
    .unwrap();

    let mut scan_data = [0; 31];
    AdStructure::encode_slice(
        &[AdStructure::CompleteLocalName(b"Trouble")],
        &mut scan_data[..],
    )
    .unwrap();

    let _ = join(runner.run(), async {
        loop {
            info!("Advertising, waiting for connection...");
            let advertiser = peripheral
                .advertise(
                    &Default::default(),
                    Advertisement::ConnectableScannableUndirected {
                        adv_data: &adv_data[..],
                        scan_data: &scan_data[..],
                    },
                )
                .await
                .unwrap();
            let conn = advertiser.accept().await.unwrap();

            info!("Connection established");

            let mut ch1 = L2capChannel::accept(&stack, &conn, &[0x2349], &Default::default())
                .await
                .unwrap();

            info!("L2CAP channel accepted");

            // Size of payload we're expecting
            const PAYLOAD_LEN: usize = 27;
            let mut rx = [0; PAYLOAD_LEN];
            for i in 0..10 {
                let len = ch1.receive(&stack, &mut rx).await.unwrap();
                assert_eq!(len, rx.len());
                assert_eq!(rx, [i; PAYLOAD_LEN]);
            }

            info!("L2CAP data received, echoing");
            Timer::after(Duration::from_secs(1)).await;
            for i in 0..10 {
                let tx = [i; PAYLOAD_LEN];
                ch1.send::<_, L2CAP_MTU>(&stack, &tx).await.unwrap();
            }
            info!("L2CAP data echoed");

            Timer::after(Duration::from_secs(60)).await;
        }
    })
    .await;
}
