use ariel_os_log::{debug, info, warn};
use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use esp_radio::wifi::{
    Config, ModeConfig, WifiController, WifiDevice, WifiEvent, WifiStationState, sta::StationConfig,
};

static WIFI_WATCH: Watch<CriticalSectionRawMutex, bool, 1> = Watch::new();
pub type NetworkDevice = WifiDevice<'static>;

pub fn connect() {
    WIFI_WATCH.sender().send(true);
    info!("connect");
}

pub fn disconnect() {
    WIFI_WATCH.sender().send(false);
    info!("disconnect");
}

pub fn init(peripherals: &mut crate::OptionalPeripherals, spawner: Spawner) -> NetworkDevice {
    let config = Config::default();
    let wifi = peripherals.WIFI.take().unwrap();

    let (controller, interfaces) = esp_radio::wifi::new(wifi, config).unwrap();
    WIFI_WATCH.sender().send(true);
    spawner.spawn(connection(controller)).ok();

    interfaces.station
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    debug!("start connection task");

    #[cfg(not(feature = "defmt"))]
    debug!("Device capabilities: {:?}", controller.capabilities());

    let mut state_receiver = WIFI_WATCH
        .receiver()
        .expect("getting a receiver for the wanted wifi state.");

    loop {
        // Wait for the state to be wanted up.
        if !state_receiver.get().await {
            state_receiver.changed().await;
            continue;
        }

        match esp_radio::wifi::station_state() {
            WifiStationState::Connected => {
                info!("Waiting for disconnection event");
                // wait until we're no longer connected
                match select(
                    controller.wait_for_event(WifiEvent::StationDisconnected),
                    state_receiver.changed(),
                )
                .await
                {
                    Either::First(_) => Timer::after(Duration::from_secs(5)).await,
                    Either::Second(_) => {
                        info!("received a link state command");
                    }
                }
            }
            _ => {}
        }
        info!("loop");

        if state_receiver.get().await {
            if !matches!(controller.is_started(), Ok(true)) {
                debug!("Configuring Wi-Fi");
                let client_config = ModeConfig::Station(
                    StationConfig::default()
                        .with_ssid(crate::wifi::WIFI_NETWORK.try_into().unwrap())
                        .with_password(crate::wifi::WIFI_PASSWORD.try_into().unwrap()),
                );
                controller.set_config(&client_config).unwrap();
                debug!("Starting Wi-Fi");
                controller.start_async().await.unwrap();
                debug!("Wi-Fi started!");
            }
            info!("About to connect...");
            if !matches!(controller.is_connected(), Ok(true)) {
                match controller.connect_async().await {
                    Ok(_) => info!("Wifi connected!"),
                    Err(e) => {
                        warn!("Failed to connect to Wi-Fi: {:?}", e);
                        Timer::after(Duration::from_millis(5000)).await
                    }
                }
            } else {
                debug!("Wi-Fi already connected");
            }
        } else {
            match controller.disconnect_async().await {
                Ok(_) => info!("Wifi disconnected!"),
                Err(e) => {
                    info!("Failed to disconnect to Wi-Fi: {:?}", e);
                }
            }
        }
    }
}
