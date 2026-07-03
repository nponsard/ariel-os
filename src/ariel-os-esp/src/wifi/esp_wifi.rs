use ariel_os_log::{debug, info};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_radio::wifi::{Config, ControllerConfig, Interface, WifiController, sta::StationConfig};

pub type NetworkDevice = Interface<'static>;

pub fn init(peripherals: &mut crate::OptionalPeripherals, spawner: Spawner) -> NetworkDevice {
    let config = ControllerConfig::default();
    let wifi = peripherals.WIFI.take().unwrap();

    // (swallowing the `WifiError` here, it pulls in quite a bit of fmt code.)
    let (controller, interfaces) = esp_radio::wifi::new(wifi, config).ok().unwrap();

    spawner.spawn(connection(controller).unwrap());

    interfaces.station
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    debug!("start connection task");

    debug!("Configuring Wi-Fi");
    let client_config = Config::Station(
        StationConfig::default()
            .with_ssid(crate::wifi::WIFI_NETWORK)
            .with_password(crate::wifi::WIFI_PASSWORD.into()),
    );

    // (swallowing the `WifiError` here, it pulls in quite a bit of fmt code.)
    controller.set_config(&client_config).ok().unwrap();

    debug!("Wi-Fi configured!");

    loop {
        debug!("About to connect...");

        match controller.connect_async().await {
            Ok(_) => {
                info!("Wifi connected!");
                // Wait until we're no longer connected, then loop to reconnect.
                let _ = controller.wait_for_disconnect_async().await;
            }
            Err(e) => {
                info!("Failed to connect to Wi-Fi: {:?}", e);
            }
        }

        Timer::after(Duration::from_secs(5)).await;
    }
}
