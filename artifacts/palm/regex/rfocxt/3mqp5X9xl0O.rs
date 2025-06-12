use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::ops::Index;
use std::str::FromStr;
use std::sync::Arc;
use memchr::memchr;
use exec::{Exec, ExecNoSync};
use expand::expand_bytes;
use error::Error;
use re_builder::bytes::RegexBuilder;
use re_trait::{self, RegularExpression, Locations, SubCapturesPosIter};
pub trait Replacer {
    fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>);
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, [u8]>> {
        None
    }
    fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self> {
        ReplacerRef(self)
    }
}
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
impl<'a> Replacer for &'a [u8] {
    fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>) {}
    fn no_expansion(&mut self) -> Option<Cow<[u8]>> {
        match memchr(b'$', *self) {
            Some(_) => None,
            None => Some(Cow::Borrowed(*self)),
        }
    }
}
