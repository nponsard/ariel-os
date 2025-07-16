use ariel_os_embassy_common::gps::Gps;

pub(crate) fn config() -> Config {
    #[cfg(not(feature = "gps-config-override"))]
    {
        Config::default()
    }
    #[cfg(feature = "gps-config-override")]
    {
        unsafe extern "Rust" {
            fn __ariel_os_gps_config() -> Config;
        }
        unsafe { __ariel_os_gps_config() }
    }
}
