use std::char;
use std::cmp::Ordering;
use std::fmt;
use std::ops;
use std::u32;
use syntax;
use literal::LiteralSearcher;
use prog::InstEmptyLook;
use utf8::{decode_utf8, decode_last_utf8};
#[derive(Clone, Copy, Debug)]
pub struct InputAt {
    pos: usize,
    c: Char,
    byte: Option<u8>,
    len: usize,
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char(u32);
impl InputAt {
    pub fn is_start(&self) -> bool {
        self.pos == 0
    }
    pub fn is_end(&self) -> bool {}
    pub fn char(&self) -> Char {}
    pub fn byte(&self) -> Option<u8> {}
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn pos(&self) -> usize {}
    pub fn next_pos(&self) -> usize {}
}
