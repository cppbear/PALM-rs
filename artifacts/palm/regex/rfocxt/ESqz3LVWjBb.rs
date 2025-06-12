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
#[derive(Clone)]
pub struct Regex(Exec);
pub struct Split<'r, 't> {
    finder: Matches<'r, 't>,
    last: usize,
}
pub struct Exec {
    /// All read only state.
    ro: Arc<ExecReadOnly>,
    /// Caches for the various matching engines.
    cache: CachedThreadLocal<ProgramCache>,
}
pub struct SplitN<'r, 't> {
    splits: Split<'r, 't>,
    n: usize,
}
impl Regex {
    pub fn new(re: &str) -> Result<Regex, Error> {}
    pub fn is_match(&self, text: &[u8]) -> bool {}
    pub fn find<'t>(&self, text: &'t [u8]) -> Option<Match<'t>> {}
    pub fn find_iter<'r, 't>(&'r self, text: &'t [u8]) -> Matches<'r, 't> {}
    pub fn captures<'t>(&self, text: &'t [u8]) -> Option<Captures<'t>> {}
    pub fn captures_iter<'r, 't>(&'r self, text: &'t [u8]) -> CaptureMatches<'r, 't> {}
    pub fn split<'r, 't>(&'r self, text: &'t [u8]) -> Split<'r, 't> {
        Split {
            finder: self.find_iter(text),
            last: 0,
        }
    }
    pub fn splitn<'r, 't>(&'r self, text: &'t [u8], limit: usize) -> SplitN<'r, 't> {
        SplitN {
            splits: self.split(text),
            n: limit,
        }
    }
    pub fn replace<'t, R: Replacer>(&self, text: &'t [u8], rep: R) -> Cow<'t, [u8]> {}
    pub fn replace_all<'t, R: Replacer>(&self, text: &'t [u8], rep: R) -> Cow<'t, [u8]> {}
    pub fn replacen<'t, R: Replacer>(
        &self,
        text: &'t [u8],
        limit: usize,
        mut rep: R,
    ) -> Cow<'t, [u8]> {}
}
