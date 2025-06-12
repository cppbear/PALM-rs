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
pub struct Exec {
    /// All read only state.
    ro: Arc<ExecReadOnly>,
    /// Caches for the various matching engines.
    cache: CachedThreadLocal<ProgramCache>,
}
#[derive(Clone, PartialEq)]
pub enum Error {
    /// A syntax error.
    Syntax(String),
    /// The compiled program exceeded the set size limit.
    /// The argument is the size limit imposed.
    CompiledTooBig(usize),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}
impl Regex {
    pub fn new(re: &str) -> Result<Regex, Error> {
        RegexBuilder::new(re).build()
    }
    pub fn is_match(&self, text: &[u8]) -> bool {}
    pub fn find<'t>(&self, text: &'t [u8]) -> Option<Match<'t>> {}
    pub fn find_iter<'r, 't>(&'r self, text: &'t [u8]) -> Matches<'r, 't> {}
    pub fn captures<'t>(&self, text: &'t [u8]) -> Option<Captures<'t>> {}
    pub fn captures_iter<'r, 't>(&'r self, text: &'t [u8]) -> CaptureMatches<'r, 't> {}
    pub fn split<'r, 't>(&'r self, text: &'t [u8]) -> Split<'r, 't> {}
    pub fn splitn<'r, 't>(&'r self, text: &'t [u8], limit: usize) -> SplitN<'r, 't> {}
    pub fn replace<'t, R: Replacer>(&self, text: &'t [u8], rep: R) -> Cow<'t, [u8]> {}
    pub fn replace_all<'t, R: Replacer>(&self, text: &'t [u8], rep: R) -> Cow<'t, [u8]> {}
    pub fn replacen<'t, R: Replacer>(
        &self,
        text: &'t [u8],
        limit: usize,
        mut rep: R,
    ) -> Cow<'t, [u8]> {}
}
