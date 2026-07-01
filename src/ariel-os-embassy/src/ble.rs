//! Provides control over the system BLE (Bluetooth Low Energy) stack.
//!
//! All interactions happen through the [`trouble_host::Stack`] struct that can be obtained using
//! [`ble_stack()`].
//!
//! The address of the device is randomly generated at boot and may be rotated during execution.
//!
//! # Current implementation
//!
//! The address is not currently rotated during execution; however this behavior may not be relied upon.

use embassy_sync::once_lock::OnceLock;
use futures_util::FutureExt as _;
use trouble_host::{
    Address,
    prelude::{AddrKind, BdAddr},
};

use ariel_os_embassy_common::ble::Config;
use ariel_os_log::debug;

// Must be async and return &trouble_host::Stack<'static, impl Controller>
pub use crate::hal::ble::ble_stack;

static CURRENT_ADDRESS: OnceLock<Address> = OnceLock::new();

#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) async fn config() -> Config {
    let address = if let Some((_, addr)) = get_bond_information().await {
        // If we have a bonded device we need to use the same address, or use a
        // resolvable private address, since we don't support the latter we use
        // the previous address.
        addr
    } else {
        // Scanning apps show that the last byte of the array appears fist.
        let mut raw_address = get_random_addr();

        // Set the two most significant bits to 1 to indicate a static random address https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-7edea27a-a47f-8436-4bd7-aedc1945c366_figure-idm4497995733171233616486354268
        raw_address[5] |= 0b1100_0000;
        // Set the two most significatn bits to 0 to indicate a private random address
        // raw_address[5] &= 0b0011_1111;

        Address {
            addr: BdAddr::new(raw_address),
            kind: AddrKind::RANDOM,
        }
    };

    let _ = CURRENT_ADDRESS.init(address);

    debug!("Setting random address: {:?}", address);

    Config { address }
}

/// Returns the BLE address currently in use.
///
/// Note that the BLE address may be rotated over time.
pub fn current_address() -> impl Future<Output = Address> {
    // Using map() to avoid creating a new state machine.
    CURRENT_ADDRESS.get().map(|addr| *addr)
}

#[cfg(feature = "ble-security")]
mod security {
    use serde::{Deserialize, Serialize};
    use trouble_host::{
        Address, BondInformation, Identity, IdentityResolvingKey, LongTermKey,
        connection::SecurityLevel, prelude::BdAddr,
    };

    use ariel_os_log::{Debug2Format, warn};
    use ariel_os_storage as storage;

    const BOND_STORAGE_KEY: &str = "BLE_BOND";
    // Storing the address the device should be reacheable at for this bond
    const BOND_ADDR_STORAGE_KEY: &str = "BLE_BOND_ADDR";

    #[derive(Serialize, Deserialize)]
    struct StoredBondInformation {
        ltk: u128,
        identity: StoredIdentity,
        is_bonded: bool,
        security_level: StoredSecurityLevel,
    }

    impl Into<BondInformation> for StoredBondInformation {
        fn into(self) -> BondInformation {
            BondInformation {
                ltk: LongTermKey(self.ltk),
                identity: self.identity.into(),
                is_bonded: self.is_bonded,
                security_level: self.security_level.into(),
            }
        }
    }

    impl From<BondInformation> for StoredBondInformation {
        fn from(bond_information: BondInformation) -> Self {
            Self {
                ltk: bond_information.ltk.0,
                identity: bond_information.identity.into(),
                is_bonded: bond_information.is_bonded,
                security_level: bond_information.security_level.into(),
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    struct StoredIdentity {
        pub bd_addr: [u8; 6],
        pub irk: Option<u128>,
    }

    impl Into<Identity> for StoredIdentity {
        fn into(self) -> Identity {
            Identity {
                bd_addr: BdAddr::new(self.bd_addr),
                irk: self.irk.map(|irk| IdentityResolvingKey(irk)),
            }
        }
    }

    impl From<Identity> for StoredIdentity {
        fn from(identiy: Identity) -> Self {
            Self {
                bd_addr: identiy.bd_addr.into_inner(),
                irk: identiy.irk.map(|irk| irk.0),
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    enum StoredSecurityLevel {
        NoEncryption,
        Encrypted,
        EncryptedAuthenticated,
    }

    impl Into<SecurityLevel> for StoredSecurityLevel {
        fn into(self) -> SecurityLevel {
            match self {
                Self::NoEncryption => SecurityLevel::NoEncryption,
                Self::Encrypted => SecurityLevel::Encrypted,
                Self::EncryptedAuthenticated => SecurityLevel::EncryptedAuthenticated,
            }
        }
    }

    impl From<SecurityLevel> for StoredSecurityLevel {
        fn from(security_level: SecurityLevel) -> Self {
            match security_level {
                SecurityLevel::NoEncryption => Self::NoEncryption,
                SecurityLevel::Encrypted => Self::Encrypted,
                SecurityLevel::EncryptedAuthenticated => Self::EncryptedAuthenticated,
            }
        }
    }

    /// Store the BLE bond information in storage to restore it on boot.
    pub async fn store_bond_information(
        bonding_information: BondInformation,
    ) -> Result<(), sequential_storage::Error<ariel_os_hal::hal::storage::FlashError>> {
        let storeable_bond: StoredBondInformation = bonding_information.into();
        let current_address = crate::ble::current_address().await;

        storage::insert(BOND_STORAGE_KEY, storeable_bond).await?;
        storage::insert(BOND_ADDR_STORAGE_KEY, current_address.addr.into_inner()).await
    }

    /// Remove the bond information from storage so it won't be restored next boot.
    pub async fn remove_bond_information()
    -> Result<(), sequential_storage::Error<ariel_os_hal::hal::storage::FlashError>> {
        storage::remove(BOND_STORAGE_KEY).await?;
        storage::remove(BOND_ADDR_STORAGE_KEY).await
    }

    /// Returns the bond information if present.
    pub async fn get_bond_information() -> Option<(BondInformation, Address)> {
        let bond_information: Option<BondInformation> = match storage::get(BOND_STORAGE_KEY).await {
            Ok(option) => option.map(|b: StoredBondInformation| b.into()),
            Err(err) => {
                warn!("Flash read error: {:?}", Debug2Format(&err));
                None
            }
        };

        if let Some(bond) = bond_information {
            match storage::get(BOND_ADDR_STORAGE_KEY).await {
                Ok(addr) => Some((bond, Address::random(addr?))),
                Err(err) => {
                    warn!("Flash read error: {:?}", Debug2Format(&err));
                    None
                }
            }
        } else {
            None
        }
    }
}
#[cfg(feature = "ble-security")]
pub use security::{
    get_bond_information, remove_bond_information, store_bond_information,
};

/// Generates a random address.
fn get_random_addr() -> [u8; 6] {
    let mut addr = [0u8; 6];
    rand_core::RngCore::fill_bytes(&mut ariel_os_random::crypto_rng(), &mut addr);
    addr
}
