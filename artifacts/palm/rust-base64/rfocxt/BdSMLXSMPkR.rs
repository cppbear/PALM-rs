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
impl<'a> Sink for StringSink<'a> {
    type Error = ();
    fn write_encoded_bytes(&mut self, s: &[u8]) -> Result<(), Self::Error> {
        self.string.push_str(str::from_utf8(s).unwrap());
        Ok(())
    }
}
