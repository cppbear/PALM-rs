use crate::BufMut;
use std::{cmp, io};
#[derive(Debug)]
pub struct Writer<B> {
    buf: B,
}
pub fn new<B>(buf: B) -> Writer<B> {
    Writer { buf }
}
