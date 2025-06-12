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
pub struct Captures<'t> {
    text: &'t str,
    locs: Locations,
    named_groups: Arc<HashMap<String, usize>>,
}
pub struct Locations(Vec<Slot>);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t str,
    start: usize,
    end: usize,
}
impl<'t> Captures<'t> {
    pub fn get(&self, i: usize) -> Option<Match<'t>> {
        self.locs.pos(i).map(|(s, e)| Match::new(self.text, s, e))
    }
    pub fn name(&self, name: &str) -> Option<Match<'t>> {}
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 't> {}
    pub fn expand(&self, replacement: &str, dst: &mut String) {}
    #[inline]
    pub fn len(&self) -> usize {}
}
impl Locations {
    pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
        let (s, e) = (i * 2, i * 2 + 1);
        match (self.0.get(s), self.0.get(e)) {
            (Some(&Some(s)), Some(&Some(e))) => Some((s, e)),
            _ => None,
        }
    }
    pub fn iter(&self) -> SubCapturesPosIter {}
    pub fn len(&self) -> usize {}
}
