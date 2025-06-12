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
    pub fn shortest_match(&self, text: &str) -> Option<usize> {}
    pub fn shortest_match_at(&self, text: &str, start: usize) -> Option<usize> {
        self.0.searcher_str().shortest_match_at(text, start)
    }
    pub fn is_match_at(&self, text: &str, start: usize) -> bool {
        self.shortest_match_at(text, start).is_some()
    }
    pub fn find_at<'t>(&self, text: &'t str, start: usize) -> Option<Match<'t>> {}
    pub fn read_captures_at<'t>(
        &self,
        locs: &mut Locations,
        text: &'t str,
        start: usize,
    ) -> Option<Match<'t>> {}
}
