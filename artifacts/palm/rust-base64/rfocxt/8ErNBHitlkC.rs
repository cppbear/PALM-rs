use super::encoder::EncoderWriter;
use crate::engine::Engine;
use std::io;
pub trait StrConsumer {
    fn consume(&mut self, buf: &str);
}
struct Utf8SingleCodeUnitWriter<S: StrConsumer> {
    str_consumer: S,
}
impl<S: StrConsumer> io::Write for Utf8SingleCodeUnitWriter<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {}
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
