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
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = std::str::from_utf8(buf).expect("Input must be valid UTF-8");
        self.str_consumer.consume(s);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {}
}
