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
struct ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
}
impl Error {
    pub fn line(&self) -> usize {
        self.err.line
    }
    pub fn column(&self) -> usize {}
    pub fn classify(&self) -> Category {}
    pub fn is_io(&self) -> bool {}
    pub fn is_syntax(&self) -> bool {}
    pub fn is_data(&self) -> bool {}
    pub fn is_eof(&self) -> bool {}
    #[cfg(feature = "std")]
    pub fn io_error_kind(&self) -> Option<ErrorKind> {}
}
