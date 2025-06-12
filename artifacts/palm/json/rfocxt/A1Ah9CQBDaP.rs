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
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}
struct JsonUnexpected<'a>(de::Unexpected<'a>);
struct ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
}
impl de::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {}
    #[cold]
    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error::custom(
            format_args!("invalid type: {}, expected {}", JsonUnexpected(unexp), exp,),
        )
    }
    #[cold]
    fn invalid_value(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error::custom(
            format_args!("invalid value: {}, expected {}", JsonUnexpected(unexp), exp,),
        )
    }
}
