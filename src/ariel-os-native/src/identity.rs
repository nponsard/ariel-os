//! Provides a Device ID based on [machine-id].
//!
//! As recommended by its documentation, it gets hashed in an application specific way.
//!
//! [machine-id]: https://www.freedesktop.org/software/systemd/man/latest/machine-id.html

use sha2::Digest as _;

const SHA256LEN: usize = 32;

pub struct DeviceId([u8; SHA256LEN]);

impl ariel_os_embassy_common::identity::DeviceId for DeviceId {
    fn get() -> Result<Self, impl std::error::Error> {
        let mut hash = sha2::Sha256::new();
        hash.update(ariel_os_buildinfo::OS_NAME);
        hash.update(b"device identity");
        // We *could* decode the hex value, but why -- the format of that file is fixed anyway.
        hash.update(std::fs::read("/etc/machine-id")?);
        let hash = hash.finalize();
        Ok::<_, std::io::Error>(DeviceId(hash.into()))
    }

    type Bytes = [u8; SHA256LEN];

    fn bytes(&self) -> Self::Bytes {
        self.0
    }

    // Not providing the `interface_eui48` method, because while there *are* MAC addresses assigned
    // to the hardware, they're generally in active use.
}
