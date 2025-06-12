use super::encoder::EncoderWriter;
use crate::engine::Engine;
use std::io;
pub trait StrConsumer {
    fn consume(&mut self, buf: &str);
}
impl StrConsumer for String {
    fn consume(&mut self, buf: &str) {
        self.push_str(buf);
    }
}
