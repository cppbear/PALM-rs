pub type Result<T> = result::Result<T, Error>;
use crate::io;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::{self, Debug, Display};
use core::result;
use core::str::FromStr;
use serde::{de, ser};
#[cfg(feature = "std")]
use std::error;
#[cfg(feature = "std")]
use std::io::ErrorKind;
struct JsonUnexpected<'a>(de::Unexpected<'a>);
impl<'a> Display for JsonUnexpected<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            de::Unexpected::Unit => formatter.write_str("null"),
            de::Unexpected::Float(value) => {
                write!(
                    formatter, "floating point `{}`", ryu::Buffer::new().format(value),
                )
            }
            unexp => Display::fmt(&unexp, formatter),
        }
    }
}
