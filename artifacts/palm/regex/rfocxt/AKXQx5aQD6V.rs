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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t [u8],
    start: usize,
    end: usize,
}
#[derive(Debug)]
pub struct ExecNoSync<'c> {
    /// All read only state.
    ro: &'c Arc<ExecReadOnly>,
    /// Caches for the various matching engines.
    cache: &'c ProgramCache,
}
impl Regex {
    pub fn shortest_match(&self, text: &[u8]) -> Option<usize> {}
    pub fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {}
    pub fn is_match_at(&self, text: &[u8], start: usize) -> bool {}
    pub fn find_at<'t>(&self, text: &'t [u8], start: usize) -> Option<Match<'t>> {
        self.0.searcher().find_at(text, start).map(|(s, e)| Match::new(text, s, e))
    }
    pub fn read_captures_at<'t>(
        &self,
        locs: &mut Locations,
        text: &'t [u8],
        start: usize,
    ) -> Option<Match<'t>> {}
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
