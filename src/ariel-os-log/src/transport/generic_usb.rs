use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pipe::Pipe};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
};

const USB_PIPE_SIZE: usize = 1024;
const MAX_USB_PACKET_SIZE: u16 = 64;

static USB_LOG_PIPE: Pipe<CriticalSectionRawMutex, USB_PIPE_SIZE> = Pipe::new();

pub(crate) fn write_bytes(bytes: &[u8]) {
    let end = bytes.len();

    let mut total = 0;
    while total < end {
        let n = match USB_LOG_PIPE.try_write(&bytes[total..end]) {
            Ok(n) => n,
            // Pipe full, drop the data.
            Err(_) => return,
        };
        total += n;
    }
}
pub(crate) fn flush() {

    // TODO: implement flush. This is complicated as the USB communication is handled in another
    // task, if we use block_on() the pipe will never be flushed as the task will never be executed.

    // USB_LOG_PIPE.flush().await;
}

pub struct UsbLoggerRunner<'d, D: embassy_usb::driver::Driver<'d>> {
    usb_cdc_acm: CdcAcmClass<'d, D>,
}

impl<'d, D: embassy_usb::driver::Driver<'d>> UsbLoggerRunner<'d, D> {
    pub async fn run(mut self) {
        let (mut sender, mut receiver) = self.usb_cdc_acm.split();
        let sender_fut = async {
            sender.wait_connection().await;
            let mut buffer = [0; MAX_USB_PACKET_SIZE as usize];
            loop {
                let len = USB_LOG_PIPE.read(&mut buffer).await;

                if Err(EndpointError::Disabled) == sender.write_packet(&buffer[..len]).await {
                    sender.wait_connection().await;
                };
            }
        };

        // We need to read from the USB otherwise some hosts won't be able to close the connection.
        let receiver_fut = async {
            receiver.wait_connection().await;
            let mut buffer = [0; MAX_USB_PACKET_SIZE as usize];
            loop {
                if Err(EndpointError::Disabled) == receiver.read_packet(&mut buffer).await {
                    receiver.wait_connection().await;
                };
            }
        };

        embassy_futures::join::join(sender_fut, receiver_fut).await;
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
