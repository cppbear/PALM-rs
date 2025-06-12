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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t str,
    start: usize,
    end: usize,
}
impl<'t> Match<'t> {
    #[inline]
    pub fn start(&self) -> usize {}
    #[inline]
    pub fn end(&self) -> usize {}
    #[inline]
    pub fn as_str(&self) -> &'t str {}
    #[inline]
    fn new(haystack: &'t str, start: usize, end: usize) -> Match<'t> {
        Match {
            text: haystack,
            start: start,
            end: end,
        }
    }
}
