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
#[derive(Debug)]
pub struct ExecNoSync<'c> {
    /// All read only state.
    ro: &'c Arc<ExecReadOnly>,
    /// Caches for the various matching engines.
    cache: &'c ProgramCache,
}
pub struct Matches<'r, 't>(re_trait::Matches<'t, ExecNoSync<'r>>);
impl Regex {
    pub fn new(re: &str) -> Result<Regex, Error> {}
    pub fn is_match(&self, text: &[u8]) -> bool {}
    pub fn find<'t>(&self, text: &'t [u8]) -> Option<Match<'t>> {}
    pub fn find_iter<'r, 't>(&'r self, text: &'t [u8]) -> Matches<'r, 't> {
        Matches(self.0.searcher().find_iter(text))
    }
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
impl Exec {
    #[inline(always)]
    pub fn searcher(&self) -> ExecNoSync {
        let create = || Box::new(RefCell::new(ProgramCacheInner::new(&self.ro)));
        ExecNoSync {
            ro: &self.ro,
            cache: self.cache.get_or(create),
        }
    }
    #[inline(always)]
    pub fn searcher_str(&self) -> ExecNoSyncStr {}
    pub fn into_regex(self) -> re_unicode::Regex {}
    pub fn into_regex_set(self) -> re_set::unicode::RegexSet {}
    pub fn into_byte_regex(self) -> re_bytes::Regex {}
    pub fn into_byte_regex_set(self) -> re_set::bytes::RegexSet {}
    pub fn regex_strings(&self) -> &[String] {}
    pub fn capture_names(&self) -> &[Option<String>] {}
    pub fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {}
}
