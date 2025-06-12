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
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char(u32);
#[derive(Clone, Copy, Debug)]
pub struct InputAt {
    pos: usize,
    c: Char,
    byte: Option<u8>,
    len: usize,
}
impl<'t> Input for CharInput<'t> {
    fn at(&self, i: usize) -> InputAt {
        let c = decode_utf8(&self[i..]).map(|(c, _)| c).into();
        InputAt {
            pos: i,
            c: c,
            byte: None,
            len: c.len_utf8(),
        }
    }
    fn next_char(&self, at: InputAt) -> Char {}
    fn previous_char(&self, at: InputAt) -> Char {}
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {}
    fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {}
    fn len(&self) -> usize {}
    fn as_bytes(&self) -> &[u8] {}
}
impl Char {
    #[inline]
    pub fn is_none(self) -> bool {}
    #[inline]
    pub fn len_utf8(self) -> usize {
        char::from_u32(self.0).map_or(0, |c| c.len_utf8())
    }
    pub fn is_word_char(self) -> bool {}
    pub fn is_word_byte(self) -> bool {}
}
#[inline]
pub fn decode_utf8(src: &[u8]) -> Option<(char, usize)> {
    let b0 = match src.get(0) {
        None => return None,
        Some(&b) if b <= 0x7F => return Some((b as char, 1)),
        Some(&b) => b,
    };
    match b0 {
        0b110_00000..=0b110_11111 => {
            if src.len() < 2 {
                return None;
            }
            let b1 = src[1];
            if 0b11_000000 & b1 != TAG_CONT {
                return None;
            }
            let cp = ((b0 & !TAG_TWO) as u32) << 6 | ((b1 & !TAG_CONT) as u32);
            match cp {
                0x80..=0x7FF => char::from_u32(cp).map(|cp| (cp, 2)),
                _ => None,
            }
        }
        0b1110_0000..=0b1110_1111 => {
            if src.len() < 3 {
                return None;
            }
            let (b1, b2) = (src[1], src[2]);
            if 0b11_000000 & b1 != TAG_CONT {
                return None;
            }
            if 0b11_000000 & b2 != TAG_CONT {
                return None;
            }
            let cp = ((b0 & !TAG_THREE) as u32) << 12 | ((b1 & !TAG_CONT) as u32) << 6
                | ((b2 & !TAG_CONT) as u32);
            match cp {
                0x800..=0xFFFF => char::from_u32(cp).map(|cp| (cp, 3)),
                _ => None,
            }
        }
        0b11110_000..=0b11110_111 => {
            if src.len() < 4 {
                return None;
            }
            let (b1, b2, b3) = (src[1], src[2], src[3]);
            if 0b11_000000 & b1 != TAG_CONT {
                return None;
            }
            if 0b11_000000 & b2 != TAG_CONT {
                return None;
            }
            if 0b11_000000 & b3 != TAG_CONT {
                return None;
            }
            let cp = ((b0 & !TAG_FOUR) as u32) << 18 | ((b1 & !TAG_CONT) as u32) << 12
                | ((b2 & !TAG_CONT) as u32) << 6 | ((b3 & !TAG_CONT) as u32);
            match cp {
                0x10000..=0x10FFFF => char::from_u32(cp).map(|cp| (cp, 4)),
                _ => None,
            }
        }
        _ => None,
    }
}
