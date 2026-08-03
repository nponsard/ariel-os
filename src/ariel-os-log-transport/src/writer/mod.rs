/// Module for log transports that implement [`embedded_io_async::Write`].
///
/// ## For implementors
///
/// Transport modules should publicly export an initialization function
/// `init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals, spawner : embassy_executor::Spawner)`
/// for `ariel-os-embassy` to call it during initialization. In this function the transport driver
/// takes the peripherals needed from [`OptionalPeripherals`] and spawns tasks using the [`Spawner`]
/// if necessary.
/// The concrete type should be exposed as `pub type WriterType` and the instance should be
/// given to [`init_writer()`] after initialization.
///
/// [`OptionalPeripherals`]: [ariel_os_hal::hal::OptionalPeripherals]
/// [`Spawner`]: [embassy_executor::Spawner]
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embedded_io_async::Write as _;

cfg_select! {
    feature = "logging-over-uart" => {
        pub(crate) mod uart;
        use uart::WriterType;
        pub use uart::init;
    }

    _ => {
        compile_error!("No valid writer transport selected.");
    }
}

static WRITER: Mutex<CriticalSectionRawMutex, Option<WriterType>> = Mutex::new(None);

pub(crate) fn init_writer(w: WriterType) {
    embassy_futures::block_on(async {
        let mut writer = WRITER.lock().await;
        writer.replace(w);
    });
    ariel_os_log::transport::register_transport_fns(write_bytes, flush);
}

pub(crate) fn write_bytes(bytes: &[u8]) {
    if let Ok(mut opt) = WRITER.try_lock()
        && let Some(writer) = opt.as_mut()
    {
        let _ = embassy_futures::block_on(writer.write_all(bytes));
    }
}

pub(crate) fn flush() {
    if let Ok(mut opt) = WRITER.try_lock()
        && let Some(writer) = opt.as_mut()
    {
        let _ = embassy_futures::block_on(writer.flush());
    }
}
