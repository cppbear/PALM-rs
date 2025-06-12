use crate::error::Error;
use crate::io;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{self, Debug, Display};
use core::mem;
use core::str;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
pub use self::index::Index;
pub use self::ser::Serializer;
pub use crate::map::Map;
pub use crate::number::Number;
#[cfg(feature = "raw_value")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw_value")))]
pub use crate::raw::{to_raw_value, RawValue};
fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}
