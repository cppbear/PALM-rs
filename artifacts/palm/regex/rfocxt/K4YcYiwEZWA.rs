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
pub trait Replacer {
    fn replace_append(&mut self, caps: &Captures, dst: &mut String);
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>>;
    fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self> {
        ReplacerRef(self)
    }
}
impl<'a, T: Input> Input for &'a T {
    fn at(&self, i: usize) -> InputAt {}
    fn next_char(&self, at: InputAt) -> Char {}
    fn previous_char(&self, at: InputAt) -> Char {}
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {}
    fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {}
    fn len(&self) -> usize {}
    fn as_bytes(&self) -> &[u8] {
        (**self).as_bytes()
    }
}
