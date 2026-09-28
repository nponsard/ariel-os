use crate::OptionalPeripherals;
use core::cell::RefCell;
use embassy_sync::blocking_mutex::{Mutex, raw::CriticalSectionRawMutex};
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use esp_bootloader_esp_idf::{
    ota::OtaImageState,
    ota_updater::{self, OtaUpdater},
    partitions::{FlashRegion, PARTITION_TABLE_MAX_LEN},
};
use esp_storage::{FlashStorage, FlashStorageError};
use static_cell::{ConstStaticCell, StaticCell};

use ariel_os_embassy_common::bootloader::{FirmwareUpdater, FirmwareUpdaterError};

// TODO: change features organisation do not need this dummy struct.
pub struct HalBootLoaderBackend;

static FLASH_STORAGE: StaticCell<FlashStorage<'static>> = StaticCell::new();
static FLASH_STORAGE_REF: Mutex<
    CriticalSectionRawMutex,
    RefCell<Option<&'static mut FlashStorage<'static>>>,
> = Mutex::new(RefCell::new(None));

// Initializes the NVMC needed for operating on the flash.
pub fn init(peripherals: &mut OptionalPeripherals) {
    let flash = peripherals.FLASH.take().unwrap();

    let storage_ref = FLASH_STORAGE.init(FlashStorage::new(flash));
    let _ = FLASH_STORAGE_REF.lock(|m| m.replace(Some(storage_ref)));
}

pub struct FirmwareUpdaterBackend {
    updater: OtaUpdater<'static, FlashStorage<'static>>,
    flash_region: Option<FlashRegion<'static, FlashStorage<'static>>>,
}

impl FirmwareUpdaterBackend {
    pub fn new(flash_storage: &'static mut FlashStorage<'static>) -> Self {
        static ALIGNED_BUFFER_STATIC_CELL: ConstStaticCell<[u8; PARTITION_TABLE_MAX_LEN]> =
            ConstStaticCell::new([0; PARTITION_TABLE_MAX_LEN]);

        let buffer = ALIGNED_BUFFER_STATIC_CELL.take();

        let updater = OtaUpdater::new(flash_storage, buffer).unwrap();

        Self {
            updater,
            flash_region: None,
        }
    }
}

impl FirmwareUpdater for FirmwareUpdaterBackend {
    type StorageError = esp_bootloader_esp_idf::partitions::Error;
    fn mark_booted(&mut self) -> Result<(), FirmwareUpdaterError<Self::StorageError>> {
        self.updater
            .set_current_ota_state(OtaImageState::Valid)
            .map_err(FirmwareUpdaterError::StorageError)
    }
    fn mark_updated(&mut self) -> Result<(), FirmwareUpdaterError<Self::StorageError>> {
        // TODO: change error type to accomodate the errors from those functions ?
        self.updater
            .activate_next_partition()
            .map_err(FirmwareUpdaterError::StorageError)?;
        self.updater
            .set_current_ota_state(OtaImageState::New)
            .map_err(FirmwareUpdaterError::StorageError)
    }

    fn read_dfu(
        &mut self,
        offset: u32,
        buffer: &mut [u8],
    ) -> Result<(), FirmwareUpdaterError<Self::StorageError>> {
        let (mut partition, _t) = self
            .updater
            .next_partition()
            .map_err(FirmwareUpdaterError::StorageError)?;
        partition
            .read(offset, buffer)
            .map_err(FirmwareUpdaterError::StorageError)
    }
    fn write_dfu(
        &mut self,
        offset: u32,
        data: &[u8],
    ) -> Result<(), FirmwareUpdaterError<Self::StorageError>> {
        // Check if we correctly booted before starting an update.
        if self
            .updater
            .current_ota_state()
            .map_err(FirmwareUpdaterError::StorageError)?
            != OtaImageState::Valid
        {
            return Err(FirmwareUpdaterError::BadState);
        }

        let (mut partition, _t) = self
            .updater
            .next_partition()
            .map_err(FirmwareUpdaterError::StorageError)?;
        partition
            .write(offset, data)
            .map_err(FirmwareUpdaterError::StorageError)
    }
}

pub fn get_firmware_updater() -> FirmwareUpdaterBackend {
    let flash_storage = FLASH_STORAGE_REF.lock(|a| a.take()).unwrap();

    FirmwareUpdaterBackend::new(flash_storage)
}
