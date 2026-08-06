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

use ariel_os_embassy_common::cell::SameExecutorCell;
use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, once_lock::OnceLock,
};
use futures_util::FutureExt as _;
use static_cell::StaticCell;
use trouble_host::{
    Address, Stack,
    prelude::{AddrKind, BdAddr, DefaultPacketPool},
};

use ariel_os_embassy_common::ble::Config;
use ariel_os_log::{debug, warn};

use crate::hal::ble::BleController;

pub type BleStack = Stack<'static, BleController, DefaultPacketPool>;

static CURRENT_ADDRESS: OnceLock<Address> = OnceLock::new();
#[allow(dead_code)]
static STACK: StaticCell<SameExecutorCell<BleStack>> = StaticCell::new();
// The stack can effectively only be taken by a single application; once taken, the Option is None.
static STACKREF: OnceLock<
    Mutex<CriticalSectionRawMutex, Option<&'static mut SameExecutorCell<BleStack>>>,
> = OnceLock::new();

#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) async fn config() -> Config {
    // Scanning apps show that the last byte of the array appears fist.
    let mut raw_address = get_random_addr();

    // Set the two most significant bits to 1 to indicate a static random address https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-7edea27a-a47f-8436-4bd7-aedc1945c366_figure-idm4497995733171233616486354268
    raw_address[5] |= 0b1100_0000;
    // Set the two most significatn bits to 0 to indicate a private random address
    // raw_address[5] &= 0b0011_1111;

    let address = Address {
        addr: BdAddr::new(raw_address),
        kind: AddrKind::RANDOM,
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
pub mod security {
    use serde::{Deserialize, Serialize};
    use trouble_host::{
        Address, BondInformation, Identity, IdentityResolvingKey, LongTermKey, PacketPool, Stack,
        connection::SecurityLevel, gatt::GattConnectionEvent, prelude::BdAddr,
    };

    use ariel_os_log::{Debug2Format, warn};
    use ariel_os_storage as storage;

    // mod private {
    //     pub trait Sealed {}
    //     impl<C, P: trouble_host::PacketPool> Sealed for trouble_host::Stack<'_, C, P> {}
    // }
    // pub trait StackWrapper: private::Sealed {
    //     fn wrapped_remove_bond_information(
    //         &self,
    //         identity: Identity,
    //     ) -> impl core::future::Future<Output = Result<(), trouble_host::Error>>;
    // }

    // impl<C: trouble_host::Controller, P: trouble_host::PacketPool> StackWrapper
    //     for trouble_host::Stack<'_, C, P>
    // {
    //     async fn wrapped_remove_bond_information(
    //         &self,
    //         identity: Identity,
    //     ) -> Result<(), trouble_host::Error> {
    //         // TODO: make a custom error type.
    //         remove_bond_information().await.unwrap();
    //         self.remove_bond_information(identity)
    //     }
    // }

    const BONDS_STORAGE_KEY: &str = "BLE_BONDS";
    // Storing the address the device should be reacheable at for this bond
    const BONDED_ADDR_STORAGE_KEY: &str = "BLE_BONDED_ADDR";

    const BOND_STORAGE_COUNT: usize = 10;

    type BondStorage = heapless::Vec<StoredBondInformation, BOND_STORAGE_COUNT>;
    type BondInfoVec = heapless::Vec<BondInformation, BOND_STORAGE_COUNT>;

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

    #[derive(Serialize, Deserialize, Clone)]
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

    /// Automatically saves the bond keys when bonded.
    pub async fn gatt_event_wrapper<'stack, 'server, P: PacketPool>(
        next: impl Future<Output = GattConnectionEvent<'stack, 'server, P>>,
    ) -> GattConnectionEvent<'stack, 'server, P> {
        let event = next.await;

        if let GattConnectionEvent::PairingComplete {
            security_level: _,
            ref bond,
        } = event
            && let Some(bond_information) = bond
        {
            let _ = store_bond_information(bond_information.clone()).await;
        }
        event
    }

    /// Store the BLE bond information in storage to restore it on boot.
    pub async fn store_bond_information(
        bonding_information: BondInformation,
    ) -> Result<(), sequential_storage::Error<ariel_os_hal::hal::storage::FlashError>> {
        let storeable_bond: StoredBondInformation = bonding_information.into();
        let current_address = crate::ble::current_address().await;

        let mut store: BondStorage = match storage::get(BONDS_STORAGE_KEY).await {
            Ok(Some(store)) => store,
            _ => BondStorage::new(),
        };

        // TODO: propagate error with custom error type
        let _ = store.push(storeable_bond);

        storage::insert(BONDS_STORAGE_KEY, store).await?;
        storage::insert(BONDED_ADDR_STORAGE_KEY, current_address.addr.into_inner()).await
    }

    /// Remove the bond information from storage so it won't be restored next boot.
    pub async fn remove_bond_information<
        C: trouble_host::Controller,
        P: trouble_host::PacketPool,
    >(
        stack: &Stack<'_, C, P>,
        identity: Identity,
    ) -> Result<(), sequential_storage::Error<ariel_os_hal::hal::storage::FlashError>> {
        // TODO: propagate error with custom error type
        let _ = stack.remove_bond_information(identity);

        let mut store: BondStorage = match storage::get(BONDS_STORAGE_KEY).await {
            Ok(Some(store)) => store,
            _ => BondStorage::new(),
        };

        store = store
            .into_iter()
            .filter(|b| identity != b.identity.clone().into())
            .collect();

        if store.is_empty() {
            storage::remove(BONDS_STORAGE_KEY).await?;
            storage::remove(BONDED_ADDR_STORAGE_KEY).await?;
        } else {
            storage::insert(BONDS_STORAGE_KEY, store).await?;
        }
        Ok(())
    }

    /// Returns the bond information if present.
    pub async fn get_bond_information() -> Option<(BondInfoVec, Address)> {
        let bond_information: Option<BondInfoVec> = match storage::get(BONDS_STORAGE_KEY).await {
            Ok(option) => option.map(|b: BondStorage| b.into_iter().map(|i| i.into()).collect()),
            Err(err) => {
                warn!("Flash read error: {:?}", Debug2Format(&err));
                None
            }
        };

        if let Some(bond) = bond_information {
            match storage::get(BONDED_ADDR_STORAGE_KEY).await {
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

/// Generates a random address.
#[cfg(not(feature = "ble-config-static-address"))]
fn get_random_addr() -> [u8; 6] {
    let mut addr = [0u8; 6];
    rand_core::RngCore::fill_bytes(&mut ariel_os_random::crypto_rng(), &mut addr);
    addr
}

/// Get random address from env.
#[cfg(feature = "ble-config-static-address")]
fn get_random_addr() -> [u8; 6] {
    use ariel_os_utils::eui48_from_env;
    eui48_from_env!(
        "CONFIG_BLE_STATIC_ADDRESS",
        "static address for BLE in format XX:XX:XX:XX:XX:XX",
    )
}

/// Returns the system ble stack.
///
/// # Panics
/// - panics if the stack was already taken
/// - panics when not called from the main executor
pub async fn ble_stack() -> &'static mut BleStack {
    STACKREF
        .get()
        .await
        .try_lock()
        .expect("Two tasks racing for lock, one would fail the main-executor check")
        .take()
        .expect("Stack was already taken")
        .get_mut_async()
        .await
        .expect("Stack needs to be taken from main executor")
}

#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) async fn init_stack(controller: crate::hal::ble::BleController, spawner: Spawner) {
    let config = config().await;

    let address = config.address;

    #[cfg(feature = "ble-security")]
    let (bonds, address) =
        if let Some((bonds, stored_address)) = security::get_bond_information().await {
            (Some(bonds), stored_address)
        } else {
            (None, address)
        };

    let mut rng = ariel_os_random::crypto_rng();

    let resources = ariel_os_embassy_common::ble::get_ble_host_resources();

    let stack = trouble_host::new(controller, resources)
        .set_random_generator_seed(&mut rng)
        .set_random_address(address);

    #[cfg(feature = "ble-security")]
    if let Some(mut bond_information_vec) = bonds {
        for bond in bond_information_vec.drain(..) {
            if let Err(_err) = stack.add_bond_information(bond) {
                warn!("Failed to add BLE bond");
            }
        }
    }

    let stackref = STACK.init(SameExecutorCell::new(stack, spawner));
    let _ = STACKREF.init(Some(stackref).into());
}
