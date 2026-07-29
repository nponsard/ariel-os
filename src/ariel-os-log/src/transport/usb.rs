use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pipe::Pipe};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
};

const USB_PIPE_SIZE: usize = 1024;
const MAX_USB_PACKET_SIZE: u16 = 64;

static USB_LOG_PIPE: Pipe<CriticalSectionRawMutex, USB_PIPE_SIZE> = Pipe::new();

#[doc(hidden)]
pub enum Error {
    Writing,
}

struct UsbTransport;

impl core::fmt::Write for UsbTransport {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let end = bytes.len();

        let mut total = 0;

        while total < end {
            let n = USB_LOG_PIPE
                .try_write(&bytes[total..end])
                .map_err(|_| core::fmt::Error)?;
            total += n;
        }
        Ok(())
    }
}

// Based on <https://blog.m-ou.se/format-args/>.
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments<'_>) {
    use core::fmt::Write as _;

    UsbTransport.write_fmt(args).unwrap();
}

#[doc(hidden)]
#[macro_export]
macro_rules! usb_println {
    ($($arg:tt)*) => {{
        #[expect(clippy::used_underscore_items, reason = "consistency with std::println")]
        $crate::transport::usb::_print(format_args!("{}\n", format_args!($($arg)*)));
    }};
}

pub struct UsbLoggerRunner<'d, D: embassy_usb::driver::Driver<'d>> {
    usb_cdc_acm: CdcAcmClass<'d, D>,
}

impl<'d, D: embassy_usb::driver::Driver<'d>> UsbLoggerRunner<'d, D> {
    pub async fn run(mut self) {
        self.usb_cdc_acm.wait_connection().await;
        let mut buffer = [0; MAX_USB_PACKET_SIZE as usize];
        loop {
            let len = USB_LOG_PIPE.read(&mut buffer).await;

            if Err(EndpointError::Disabled) == self.usb_cdc_acm.write_packet(&buffer[..len]).await {
                self.usb_cdc_acm.wait_connection().await;
            };
        }
    }
}

pub fn init_usb_logger<D: embassy_usb::driver::Driver<'static>>(
    usb_builder: &mut embassy_usb::Builder<'static, D>,
) -> UsbLoggerRunner<'static, D> {
    static CDC_ACM_STATE: static_cell::StaticCell<State<'_>> = static_cell::StaticCell::new();

    // Create classes on the builder.
    let usb_cdc_acm = CdcAcmClass::new(
        usb_builder,
        CDC_ACM_STATE.init_with(State::new),
        MAX_USB_PACKET_SIZE,
    );
    UsbLoggerRunner { usb_cdc_acm }
}
