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
pub struct ByteInput<'t> {
    text: &'t [u8],
    only_utf8: bool,
}
#[derive(Clone, Copy, Debug)]
pub struct InputAt {
    pos: usize,
    c: Char,
    byte: Option<u8>,
    len: usize,
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char(u32);
#[derive(Clone, Debug)]
pub struct InstEmptyLook {
    /// The next location to execute in the program if this instruction
    /// succeeds.
    pub goto: InstPtr,
    /// The type of zero-width assertion to check.
    pub look: EmptyLook,
}
impl<'t> Input for ByteInput<'t> {
    fn at(&self, i: usize) -> InputAt {}
    fn next_char(&self, at: InputAt) -> Char {}
    fn previous_char(&self, at: InputAt) -> Char {}
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
        use prog::EmptyLook::*;
        match empty.look {
            StartLine => {
                let c = self.previous_char(at);
                at.pos() == 0 || c == '\n'
            }
            EndLine => {
                let c = self.next_char(at);
                at.pos() == self.len() || c == '\n'
            }
            StartText => at.pos() == 0,
            EndText => at.pos() == self.len(),
            WordBoundary => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_word_char() != c2.is_word_char()
            }
            NotWordBoundary => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_word_char() == c2.is_word_char()
            }
            WordBoundaryAscii => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                if self.only_utf8 {
                    if c1.is_none() && !at.is_start() {
                        return false;
                    }
                    if c2.is_none() && !at.is_end() {
                        return false;
                    }
                }
                c1.is_word_byte() != c2.is_word_byte()
            }
            NotWordBoundaryAscii => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                if self.only_utf8 {
                    if c1.is_none() && !at.is_start() {
                        return false;
                    }
                    if c2.is_none() && !at.is_end() {
                        return false;
                    }
                }
                c1.is_word_byte() == c2.is_word_byte()
            }
        }
    }
    fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {}
    fn len(&self) -> usize {}
    fn as_bytes(&self) -> &[u8] {}
}
impl InputAt {
    pub fn is_start(&self) -> bool {
        self.pos == 0
    }
    pub fn is_end(&self) -> bool {
        self.c.is_none() && self.byte.is_none()
    }
    pub fn char(&self) -> Char {}
    pub fn byte(&self) -> Option<u8> {}
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn pos(&self) -> usize {
        self.pos
    }
    pub fn next_pos(&self) -> usize {}
}
impl Char {
    #[inline]
    pub fn is_none(self) -> bool {
        self.0 == u32::MAX
    }
    #[inline]
    pub fn len_utf8(self) -> usize {}
    pub fn is_word_char(self) -> bool {
        char::from_u32(self.0).map_or(false, syntax::is_word_character)
    }
    pub fn is_word_byte(self) -> bool {
        match char::from_u32(self.0) {
            Some(c) if c <= '\u{7F}' => syntax::is_word_byte(c as u8),
            None | Some(_) => false,
        }
    }
}
