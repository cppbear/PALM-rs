use super::chunked_encoder::ChunkedEncoder;
use crate::engine::Engine;
use core::fmt::{Display, Formatter};
use core::{fmt, str};
pub trait Sink {
    type Error;
    fn write_encoded_bytes(&mut self, encoded: &[u8]) -> Result<(), Self::Error>;
}
struct FormatterSink<'a, 'b: 'a> {
    f: &'a mut Formatter<'b>,
}
impl<'a, 'b: 'a> super::chunked_encoder::Sink for FormatterSink<'a, 'b> {
    type Error = fmt::Error;
    fn write_encoded_bytes(&mut self, encoded: &[u8]) -> Result<(), Self::Error> {
        self.f.write_str(str::from_utf8(encoded).expect("base64 data was not utf8"))
    }
}
