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
    pub fn new(re: &str) -> Result<Regex, Error> {}
    pub fn is_match(&self, text: &[u8]) -> bool {}
    pub fn find<'t>(&self, text: &'t [u8]) -> Option<Match<'t>> {}
    pub fn find_iter<'r, 't>(&'r self, text: &'t [u8]) -> Matches<'r, 't> {}
    pub fn captures<'t>(&self, text: &'t [u8]) -> Option<Captures<'t>> {}
    pub fn captures_iter<'r, 't>(&'r self, text: &'t [u8]) -> CaptureMatches<'r, 't> {}
    pub fn split<'r, 't>(&'r self, text: &'t [u8]) -> Split<'r, 't> {}
    pub fn splitn<'r, 't>(&'r self, text: &'t [u8], limit: usize) -> SplitN<'r, 't> {}
    pub fn replace<'t, R: Replacer>(&self, text: &'t [u8], rep: R) -> Cow<'t, [u8]> {
        self.replacen(text, 1, rep)
    }
    pub fn replace_all<'t, R: Replacer>(&self, text: &'t [u8], rep: R) -> Cow<'t, [u8]> {}
    pub fn replacen<'t, R: Replacer>(
        &self,
        text: &'t [u8],
        limit: usize,
        mut rep: R,
    ) -> Cow<'t, [u8]> {
        if let Some(rep) = rep.no_expansion() {
            let mut it = self.find_iter(text).enumerate().peekable();
            if it.peek().is_none() {
                return Cow::Borrowed(text);
            }
            let mut new = Vec::with_capacity(text.len());
            let mut last_match = 0;
            for (i, m) in it {
                if limit > 0 && i >= limit {
                    break;
                }
                new.extend_from_slice(&text[last_match..m.start()]);
                new.extend_from_slice(&rep);
                last_match = m.end();
            }
            new.extend_from_slice(&text[last_match..]);
            return Cow::Owned(new);
        }
        let mut it = self.captures_iter(text).enumerate().peekable();
        if it.peek().is_none() {
            return Cow::Borrowed(text);
        }
        let mut new = Vec::with_capacity(text.len());
        let mut last_match = 0;
        for (i, cap) in it {
            if limit > 0 && i >= limit {
                break;
            }
            let m = cap.get(0).unwrap();
            new.extend_from_slice(&text[last_match..m.start()]);
            rep.replace_append(&cap, &mut new);
            last_match = m.end();
        }
        new.extend_from_slice(&text[last_match..]);
        Cow::Owned(new)
    }
}
