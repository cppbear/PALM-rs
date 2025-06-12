use crate::lib::*;
#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn from_utf8_lossy(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).unwrap_or("\u{fffd}\u{fffd}\u{fffd}")
}
