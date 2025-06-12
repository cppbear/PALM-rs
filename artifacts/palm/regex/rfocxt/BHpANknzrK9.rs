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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t [u8],
    start: usize,
    end: usize,
}
impl<'t> Match<'t> {
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }
    #[inline]
    pub fn end(&self) -> usize {}
    #[inline]
    pub fn as_bytes(&self) -> &'t [u8] {}
    #[inline]
    fn new(haystack: &'t [u8], start: usize, end: usize) -> Match<'t> {}
}
