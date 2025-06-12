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
impl Regex {
    pub fn shortest_match(&self, text: &[u8]) -> Option<usize> {}
    pub fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {
        self.0.searcher().shortest_match_at(text, start)
    }
    pub fn is_match_at(&self, text: &[u8], start: usize) -> bool {
        self.shortest_match_at(text, start).is_some()
    }
    pub fn find_at<'t>(&self, text: &'t [u8], start: usize) -> Option<Match<'t>> {}
    pub fn read_captures_at<'t>(
        &self,
        locs: &mut Locations,
        text: &'t [u8],
        start: usize,
    ) -> Option<Match<'t>> {}
}
