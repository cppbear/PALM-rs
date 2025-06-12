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
#[derive(Clone)]
pub struct Regex(Exec);
pub struct SplitN<'r, 't> {
    splits: Split<'r, 't>,
    n: usize,
}
pub struct Exec {
    /// All read only state.
    ro: Arc<ExecReadOnly>,
    /// Caches for the various matching engines.
    cache: CachedThreadLocal<ProgramCache>,
}
pub struct Split<'r, 't> {
    finder: Matches<'r, 't>,
    last: usize,
}
impl Regex {
    pub fn new(re: &str) -> Result<Regex, Error> {}
    pub fn is_match(&self, text: &str) -> bool {}
    pub fn find<'t>(&self, text: &'t str) -> Option<Match<'t>> {}
    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't> {}
    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {}
    pub fn captures_iter<'r, 't>(&'r self, text: &'t str) -> CaptureMatches<'r, 't> {}
    pub fn split<'r, 't>(&'r self, text: &'t str) -> Split<'r, 't> {
        Split {
            finder: self.find_iter(text),
            last: 0,
        }
    }
    pub fn splitn<'r, 't>(&'r self, text: &'t str, limit: usize) -> SplitN<'r, 't> {
        SplitN {
            splits: self.split(text),
            n: limit,
        }
    }
    pub fn replace<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {}
    pub fn replace_all<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {}
    pub fn replacen<'t, R: Replacer>(
        &self,
        text: &'t str,
        limit: usize,
        mut rep: R,
    ) -> Cow<'t, str> {}
}
