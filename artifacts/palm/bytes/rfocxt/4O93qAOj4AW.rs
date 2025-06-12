use crate::Buf;
use std::{cmp, io};
#[derive(Debug)]
pub struct Reader<B> {
    buf: B,
}
pub fn new<B>(buf: B) -> Reader<B> {
    Reader { buf }
}
