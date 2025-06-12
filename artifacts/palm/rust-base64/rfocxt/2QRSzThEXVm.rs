use crate::{encode::add_padding, engine::{Config, Engine}};
#[cfg(any(feature = "alloc", test))]
use alloc::string::String;
#[cfg(any(feature = "alloc", test))]
use core::str;
pub trait Sink {
    type Error;
    fn write_encoded_bytes(&mut self, encoded: &[u8]) -> Result<(), Self::Error>;
}
#[cfg(any(feature = "alloc", test))]
pub(crate) struct StringSink<'a> {
    string: &'a mut String,
}
#[cfg(any(feature = "alloc", test))]
impl<'a> StringSink<'a> {
    pub(crate) fn new(s: &mut String) -> StringSink {
        StringSink { string: s }
    }
}
