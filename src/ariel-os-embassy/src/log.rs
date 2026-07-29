// TODO: separate feature

#[cfg(feature = "usb")]
#[embassy_executor::task]
pub(crate) async fn usb_log_task(
    runner: ariel_os_log::transport::usb::UsbLoggerRunner<'static, crate::hal::usb::UsbDriver>,
) {
    runner.run().await;
}
