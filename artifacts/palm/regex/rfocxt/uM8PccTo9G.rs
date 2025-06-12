use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::ops::Index;
use std::str::FromStr;
use std::sync::Arc;
use memchr::memchr;
use syntax;
use error::Error;
use exec::{Exec, ExecNoSyncStr};
use expand::expand_str;
use re_builder::unicode::RegexBuilder;
use re_trait::{self, RegularExpression, Locations, SubCapturesPosIter};
pub trait Replacer {
    fn replace_append(&mut self, caps: &Captures, dst: &mut String);
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
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
impl<'a> Replacer for &'a str {
    fn replace_append(&mut self, caps: &Captures, dst: &mut String) {}
    fn no_expansion(&mut self) -> Option<Cow<str>> {
        match memchr(b'$', self.as_bytes()) {
            Some(_) => None,
            None => Some(Cow::Borrowed(*self)),
        }
    }
}
