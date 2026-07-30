use embassy_sync::once_lock::OnceLock;

static TRANSPORT_WRITE_BYTES_FN: OnceLock<fn(&[u8])> = OnceLock::new();
static TRANSPORT_FLUSH_FN: OnceLock<fn()> = OnceLock::new();

/// Register custom transport functions.
pub fn register_transport_fns(write_bytes_fn: fn(&[u8]), flush_fn: fn()) {
    let _ = TRANSPORT_WRITE_BYTES_FN.init(write_bytes_fn);
    let _ = TRANSPORT_FLUSH_FN.init(flush_fn);
}

pub(crate) fn write_bytes(bytes: &[u8]) {
    if let Some(write_fn) = TRANSPORT_WRITE_BYTES_FN.try_get() {
        write_fn(bytes);
    }
}

#[cfg(feature = "defmt")]
pub(crate) fn flush() {
    if let Some(flush_fn) = TRANSPORT_FLUSH_FN.try_get() {
        flush_fn();
    }
}
