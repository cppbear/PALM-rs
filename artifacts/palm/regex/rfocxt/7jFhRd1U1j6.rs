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
pub struct Exec {
    /// All read only state.
    ro: Arc<ExecReadOnly>,
    /// Caches for the various matching engines.
    cache: CachedThreadLocal<ProgramCache>,
}
pub struct Matches<'r, 't>(re_trait::Matches<'t, ExecNoSyncStr<'r>>);
pub struct ExecNoSyncStr<'c>(ExecNoSync<'c>);
impl Regex {
    pub fn new(re: &str) -> Result<Regex, Error> {}
    pub fn is_match(&self, text: &str) -> bool {}
    pub fn find<'t>(&self, text: &'t str) -> Option<Match<'t>> {}
    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't> {
        Matches(self.0.searcher_str().find_iter(text))
    }
    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {}
    pub fn captures_iter<'r, 't>(&'r self, text: &'t str) -> CaptureMatches<'r, 't> {}
    pub fn split<'r, 't>(&'r self, text: &'t str) -> Split<'r, 't> {}
    pub fn splitn<'r, 't>(&'r self, text: &'t str, limit: usize) -> SplitN<'r, 't> {}
    pub fn replace<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {}
    pub fn replace_all<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {}
    pub fn replacen<'t, R: Replacer>(
        &self,
        text: &'t str,
        limit: usize,
        mut rep: R,
    ) -> Cow<'t, str> {}
}
impl Exec {
    #[inline(always)]
    pub fn searcher(&self) -> ExecNoSync {}
    #[inline(always)]
    pub fn searcher_str(&self) -> ExecNoSyncStr {
        ExecNoSyncStr(self.searcher())
    }
    pub fn into_regex(self) -> re_unicode::Regex {}
    pub fn into_regex_set(self) -> re_set::unicode::RegexSet {}
    pub fn into_byte_regex(self) -> re_bytes::Regex {}
    pub fn into_byte_regex_set(self) -> re_set::bytes::RegexSet {}
    pub fn regex_strings(&self) -> &[String] {}
    pub fn capture_names(&self) -> &[Option<String>] {}
    pub fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {}
}
