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
impl Regex {
    pub fn as_str(&self) -> &str {}
    pub fn capture_names(&self) -> CaptureNames {}
    pub fn captures_len(&self) -> usize {
        self.0.capture_names().len()
    }
    pub fn locations(&self) -> Locations {}
}
impl Exec {
    #[inline(always)]
    pub fn searcher(&self) -> ExecNoSync {}
    #[inline(always)]
    pub fn searcher_str(&self) -> ExecNoSyncStr {}
    pub fn into_regex(self) -> re_unicode::Regex {}
    pub fn into_regex_set(self) -> re_set::unicode::RegexSet {}
    pub fn into_byte_regex(self) -> re_bytes::Regex {}
    pub fn into_byte_regex_set(self) -> re_set::bytes::RegexSet {}
    pub fn regex_strings(&self) -> &[String] {}
    pub fn capture_names(&self) -> &[Option<String>] {
        &self.ro.nfa.captures
    }
    pub fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {}
}
