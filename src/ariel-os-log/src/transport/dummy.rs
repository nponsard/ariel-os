// Write bytes to logging transport.
// Must not fail.
pub(crate) fn write_bytes(_bytes: &[u8]) {}
// Flush logging transport.
// Must not fail.
#[allow(unused, reason = "conditional compilation")]
pub(crate) fn flush() {}
