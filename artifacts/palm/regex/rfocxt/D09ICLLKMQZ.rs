use std::char;
use std::cmp::Ordering;
use std::fmt;
use std::ops;
use std::u32;
use syntax;
use literal::LiteralSearcher;
use prog::InstEmptyLook;
use utf8::{decode_utf8, decode_last_utf8};
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char(u32);
impl Char {
    #[inline]
    pub fn is_none(self) -> bool {}
    #[inline]
    pub fn len_utf8(self) -> usize {}
    pub fn is_word_char(self) -> bool {
        char::from_u32(self.0).map_or(false, syntax::is_word_character)
    }
    pub fn is_word_byte(self) -> bool {}
}
