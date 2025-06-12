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
pub struct Captures<'t> {
    text: &'t [u8],
    locs: Locations,
    named_groups: Arc<HashMap<String, usize>>,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t [u8],
    start: usize,
    end: usize,
}
pub struct Locations(Vec<Slot>);
impl<'t> Captures<'t> {
    pub fn get(&self, i: usize) -> Option<Match<'t>> {}
    pub fn name(&self, name: &str) -> Option<Match<'t>> {
        self.named_groups.get(name).and_then(|&i| self.get(i))
    }
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 't> {}
    pub fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>) {}
    #[inline]
    pub fn len(&self) -> usize {}
}
