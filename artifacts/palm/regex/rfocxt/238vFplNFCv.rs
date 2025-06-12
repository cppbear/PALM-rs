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
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn as_bytes(&self) -> &[u8];
}
#[derive(Clone, Copy, Debug)]
pub struct CharInput<'t>(&'t [u8]);
#[derive(Clone, Copy, Debug)]
pub struct InputAt {
    pos: usize,
    c: Char,
    byte: Option<u8>,
    len: usize,
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char(u32);
impl<'t> Input for CharInput<'t> {
    fn at(&self, i: usize) -> InputAt {}
    fn next_char(&self, at: InputAt) -> Char {
        at.char()
    }
    fn previous_char(&self, at: InputAt) -> Char {}
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {}
    fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {}
    fn len(&self) -> usize {}
    fn as_bytes(&self) -> &[u8] {}
}
impl InputAt {
    pub fn is_start(&self) -> bool {}
    pub fn is_end(&self) -> bool {}
    pub fn char(&self) -> Char {
        self.c
    }
    pub fn byte(&self) -> Option<u8> {}
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn pos(&self) -> usize {}
    pub fn next_pos(&self) -> usize {}
}
