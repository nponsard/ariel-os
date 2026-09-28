use ariel_os_embassy_common::bootloader::{
    BootloaderPartitions, BootloaderStorage, FirmwareUpdater, FlashConfig,
};
use ariel_os_hal::hal::bootloader::HalBootLoaderBackend;
use ariel_os_rt::memory::sections;
use static_cell::ConstStaticCell;

pub struct FirmwareUpdaterBackend {
    embassy_boot_updater: embassy_boot::FirmwareUpdater<
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
        > = ConstStaticCell::new(Default::default());

        let aligned = ALIGNED_BUFFER_STATIC_CELL.take();

        let embassy_boot_updater = embassy_boot::FirmwareUpdater::new(config, aligned);
        Self {
            embassy_boot_updater,
        }
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
