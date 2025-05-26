use crate::irqs::Irqs;
use ariel_os_debug::log::debug;
use embassy_executor::Spawner;
use embassy_nrf::peripherals;
use embassy_nrf::peripherals::RNG;
use embassy_nrf::rng;
use embassy_sync::once_lock::OnceLock;

use static_cell::StaticCell;
use trouble_host::prelude::*;

use apache_nimble::controller::NimbleController;
use apache_nimble::controller::NimbleControllerTask;
use embassy_time::{Duration, Ticker, Timer};

// #[embassy_executor::task]
// async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
//     debug!("Starting MPSL task");
//     mpsl.run().await
// }

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

    // pub rng: peripherals::RNG,
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

            // rng: peripherals.RNG.take().unwrap(),
        }
    }
}

/// How many outgoing L2CAP buffers per link
const L2CAP_TXQ: u8 = 3;

/// How many incoming L2CAP buffers per link
const L2CAP_RXQ: u8 = 3;

/// Size of L2CAP packets
const L2CAP_MTU: usize = 27;

#[embassy_executor::task]
async fn run_controller(controller_task: NimbleControllerTask) {
    controller_task.run().await
}
pub fn driver<'d>(
    p: Peripherals,
    spawner: &Spawner,
) -> Stack<'static, NimbleController> {
    apache_nimble::initialize_nimble();
    let controller = NimbleController::new();

    spawner
        .spawn(run_controller(controller.create_task()))
        .unwrap();

    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xcc]);

    debug!("BLE address set");

    static HOST_RESOURCES: StaticCell<HostResources<1, 2, L2CAP_MTU>> = StaticCell::new();

    let resources = HOST_RESOURCES.init(HostResources::new());

    debug!("creating stack");
    trouble_host::new(controller, resources).set_random_address(address)
}
