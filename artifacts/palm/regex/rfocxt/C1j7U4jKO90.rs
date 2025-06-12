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
pub struct Locations(Vec<Slot>);
impl<'t> Captures<'t> {
    pub fn get(&self, i: usize) -> Option<Match<'t>> {}
    pub fn name(&self, name: &str) -> Option<Match<'t>> {}
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 't> {}
    pub fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>) {}
    #[inline]
    pub fn len(&self) -> usize {
        self.locs.len()
    }
}
impl Locations {
    pub fn pos(&self, i: usize) -> Option<(usize, usize)> {}
    pub fn iter(&self) -> SubCapturesPosIter {}
    pub fn len(&self) -> usize {
        self.0.len() / 2
    }
}
