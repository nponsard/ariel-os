use ariel_os_embassy_common::cellular_networking::{
    AuthenticationProtocol, PdConfig, PdnAuth, PdnCredentials, PdpType,
};

#[cfg(all(feature = "ipv4", feature = "ipv6"))]
const PDP_TYPE: PdpType = PdpType::IpV4V6;
#[cfg(all(feature = "ipv4", not(feature = "ipv6")))]
const PDP_TYPE: PdpType = PdpType::Ip;
#[cfg(all(not(feature = "ipv4"), feature = "ipv6"))]
const PDP_TYPE: PdpType = PdpType::IpV4V6;
#[cfg(not(any(feature = "ipv4", feature = "ipv6")))]
const PDP_TYPE: PdpType = PdpType::NonIp;

const fn auth_protocol_from_str(str: &str) -> Option<AuthenticationProtocol> {
    if const_str::equal!(str, "NONE") {
        Some(AuthenticationProtocol::None)
    } else if const_str::equal!(str, "PAP") {
        Some(AuthenticationProtocol::Pap)
    } else if const_str::equal!(str, "CHAP") {
        Some(AuthenticationProtocol::Chap)
    } else {
        None
    }
}
const PIN: Option<&'static str> = option_env!("CONFIG_SIM_PIN");
const CONFIG: PdConfig<'static> = {
    let apn = option_env!("CONFIG_CELLULAR_PDN_APN");
    let authentication_protocol = option_env!("CONFIG_CELLULAR_PDN_AUTHENTICATION_PROTOCOL");
    let username = option_env!("CONFIG_CELLULAR_PDN_USERNAME");
    let password = option_env!("CONFIG_CELLULAR_PDN_PASSWORD");

    let credentials = if let Some(username) = username {
        if let Some(password) = password {
            Some(PdnCredentials { username, password })
        } else {
            panic!(
                "If you set CONFIG_CELLULAR_PDN_USERNAME you also need to set CONFIG_CELLULAR_PDN_PASSWORD"
            );
        }
    } else {
        None
    };

    let pdn_auth = if let Some(authentication_protocol) = authentication_protocol {
        let authentication_protocol = auth_protocol_from_str(authentication_protocol)
            .expect("Invalid value for CONFIG_CELLULAR_PDN_AUTHENTICATION_PROTOCOL");
        Some(PdnAuth {
            authentication_protocol,
            credentials,
        })
    } else {
        None
    };

    PdConfig {
        pdn_auth,
        apn,
        pdp_type: PDP_TYPE,
    }
};

/// Returns the configuration to authenticate to the cell network
pub fn config() -> PdConfig<'static> {
    CONFIG
}

/// Returns the pin, if set, for the SIM
pub fn pin() -> Option<&'static str> {
    PIN
}
