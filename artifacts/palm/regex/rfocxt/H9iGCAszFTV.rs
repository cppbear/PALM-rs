use std::char;
use std::cmp::Ordering;
use std::fmt;
use std::ops;
use std::u32;
use syntax;
use literal::LiteralSearcher;
use prog::InstEmptyLook;
use utf8::{decode_utf8, decode_last_utf8};
pub trait Input {
    fn at(&self, i: usize) -> InputAt;
    fn next_char(&self, at: InputAt) -> Char;
    fn previous_char(&self, at: InputAt) -> Char;
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool;
    fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn as_bytes(&self) -> &[u8];
}
#[derive(Clone, Copy, Debug)]
pub struct ByteInput<'t> {
    text: &'t [u8],
    only_utf8: bool,
}
impl<'t> ByteInput<'t> {
    pub fn new(text: &'t [u8], only_utf8: bool) -> ByteInput<'t> {
        ByteInput {
            text: text,
            only_utf8: only_utf8,
        }
    }
}
