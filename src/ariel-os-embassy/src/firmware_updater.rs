#[cfg(feature = "embassy-boot-application")]
pub use embassy_boot_application::{FirmwareUpdaterBackend, get_firmware_updater};

#[cfg(not(feature = "embassy-boot-application"))]
pub use ariel_os_hal::bootloader::{FirmwareUpdaterBackend, get_firmware_updater};

#[cfg(feature = "embassy-boot-application")]
mod embassy_boot_application {
    use ariel_os_embassy_common::bootloader::{
        BootloaderPartitions, BootloaderStorage, FirmwareUpdater, FirmwareUpdaterError, FlashConfig,
    };
    use ariel_os_hal::hal::bootloader::HalBootLoaderBackend;
    use ariel_os_rt::memory::sections;
    use static_cell::ConstStaticCell;

    pub struct FirmwareUpdaterBackend {
        embassy_boot_updater: embassy_boot::BlockingFirmwareUpdater<
            'static,
            <HalBootLoaderBackend as BootloaderStorage>::DFU,
            <HalBootLoaderBackend as BootloaderStorage>::STATE,
        >,
    }

    impl FirmwareUpdaterBackend {
        fn new<ACTIVE>(
            partitions: BootloaderPartitions<
                <HalBootLoaderBackend as BootloaderStorage>::ACTIVE,
                <HalBootLoaderBackend as BootloaderStorage>::DFU,
                <HalBootLoaderBackend as BootloaderStorage>::STATE,
            >,
        ) -> Self {
            let config = embassy_boot::FirmwareUpdaterConfig {
                dfu: partitions.dfu,
                state: partitions.bootloader_state,
            };

            static ALIGNED_BUFFER_STATIC_CELL: ConstStaticCell<
                [u8; <HalBootLoaderBackend as BootloaderStorage>::ALIGNED_BUFFER_SIZE],
            > = ConstStaticCell::new(
                [0; <HalBootLoaderBackend as BootloaderStorage>::ALIGNED_BUFFER_SIZE],
            );

            let aligned = ALIGNED_BUFFER_STATIC_CELL.take();

            let embassy_boot_updater = embassy_boot::BlockingFirmwareUpdater::new(config, aligned);
            Self {
                embassy_boot_updater,
            }
        }
    }

    impl FirmwareUpdater for FirmwareUpdaterBackend {
        type StorageError = embassy_boot::FirmwareUpdaterError;

        fn mark_updated(&mut self) -> Result<(), FirmwareUpdaterError<Self::StorageError>> {
            self.embassy_boot_updater
                .mark_updated()
                .map_err(FirmwareUpdaterError::StorageError)
        }
        fn mark_booted(&mut self) -> Result<(), FirmwareUpdaterError<Self::StorageError>> {
            self.embassy_boot_updater
                .mark_booted()
                .map_err(FirmwareUpdaterError::StorageError)
        }
        fn read_dfu(
            &mut self,
            offset: u32,
            buffer: &mut [u8],
        ) -> Result<(), FirmwareUpdaterError<Self::StorageError>> {
            self.embassy_boot_updater
                .read_dfu(offset, buffer)
                .map_err(FirmwareUpdaterError::StorageError)
        }
        fn write_dfu(
            &mut self,
            offset: u32,
            data: &[u8],
        ) -> Result<(), FirmwareUpdaterError<Self::StorageError>> {
            self.embassy_boot_updater
                .write_firmware(offset as usize, data)
                .map_err(|e| {
                    if matches!(e, embassy_boot::FirmwareUpdaterError::BadState) {
                        FirmwareUpdaterError::BadState
                    } else {
                        FirmwareUpdaterError::StorageError(e)
                    }
                })
        }
    }

    pub fn get_firmware_updater() -> FirmwareUpdaterBackend {
        let flash_config = FlashConfig {
            active: sections::ACTIVE,
            dfu: sections::DFU,
            bootloader_state: sections::BOOTLOADER_STATE,
        };

        let partitions = HalBootLoaderBackend::partitions(&flash_config);

        FirmwareUpdaterBackend::new(partitions)
    }
}
