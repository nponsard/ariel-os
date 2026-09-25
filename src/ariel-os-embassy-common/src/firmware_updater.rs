pub enum FirmwareUpdaterError<STORAGE_ERROR: Error> {
    /// Operation Was attempted while in a bad state.
    BadState,
    /// An error happened when interacting with the storage medium.
    StorageError(STORAGE_ERROR),
}

pub trait FirmwareUpdater {
    type StorageError: Error;

    /// Mark current firmware as successfully booted.
    /// Preventing the bootloader from rolling back the update.
    fn mark_booted(&mut self) -> Result<(), FirmwareUpdaterError<StorageError>>;
    /// Indicate that the new firmware has been written to the DFU slot, it will applied next boot.
    fn mark_updated(&mut self) -> Result<(), FirmwareUpdaterError<StorageError>>;
    /// Write to the DFU storage area, errors out if unaligned or out of bounds.
    fn write_dfu(
        &mut self,
        offset: usize,
        data: &[u8],
    ) -> Result<(), FirmwareUpdaterError<StorageError>>;
    /// Read from the DFU storage area, errors out if unaligned or out of bounds.
    fn read_dfu(
        &mut self,
        offset: usize,
        buffer: &mut [u8],
    ) -> Result<(), FirmwareUpdaterError<StorageError>>;
}
