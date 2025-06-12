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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t str,
    start: usize,
    end: usize,
}
pub struct Captures<'t> {
    text: &'t str,
    locs: Locations,
    named_groups: Arc<HashMap<String, usize>>,
}
pub struct Matches<'r, 't>(re_trait::Matches<'t, ExecNoSyncStr<'r>>);
pub struct CaptureMatches<'r, 't>(re_trait::CaptureMatches<'t, ExecNoSyncStr<'r>>);
pub struct Exec {
    /// All read only state.
    ro: Arc<ExecReadOnly>,
    /// Caches for the various matching engines.
    cache: CachedThreadLocal<ProgramCache>,
}
impl Regex {
    pub fn new(re: &str) -> Result<Regex, Error> {}
    pub fn is_match(&self, text: &str) -> bool {}
    pub fn find<'t>(&self, text: &'t str) -> Option<Match<'t>> {}
    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't> {
        Matches(self.0.searcher_str().find_iter(text))
    }
    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {}
    pub fn captures_iter<'r, 't>(&'r self, text: &'t str) -> CaptureMatches<'r, 't> {
        CaptureMatches(self.0.searcher_str().captures_iter(text))
    }
    pub fn split<'r, 't>(&'r self, text: &'t str) -> Split<'r, 't> {}
    pub fn splitn<'r, 't>(&'r self, text: &'t str, limit: usize) -> SplitN<'r, 't> {}
    pub fn replace<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {}
    pub fn replace_all<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {}
    pub fn replacen<'t, R: Replacer>(
        &self,
        text: &'t str,
        limit: usize,
        mut rep: R,
    ) -> Cow<'t, str> {
        if let Some(rep) = rep.no_expansion() {
            let mut it = self.find_iter(text).enumerate().peekable();
            if it.peek().is_none() {
                return Cow::Borrowed(text);
            }
            let mut new = String::with_capacity(text.len());
            let mut last_match = 0;
            for (i, m) in it {
                if limit > 0 && i >= limit {
                    break;
                }
                new.push_str(&text[last_match..m.start()]);
                new.push_str(&rep);
                last_match = m.end();
            }
            new.push_str(&text[last_match..]);
            return Cow::Owned(new);
        }
        let mut it = self.captures_iter(text).enumerate().peekable();
        if it.peek().is_none() {
            return Cow::Borrowed(text);
        }
        let mut new = String::with_capacity(text.len());
        let mut last_match = 0;
        for (i, cap) in it {
            if limit > 0 && i >= limit {
                break;
            }
            let m = cap.get(0).unwrap();
            new.push_str(&text[last_match..m.start()]);
            rep.replace_append(&cap, &mut new);
            last_match = m.end();
        }
        new.push_str(&text[last_match..]);
        Cow::Owned(new)
    }
}
impl<'t> Match<'t> {
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }
    #[inline]
    pub fn as_str(&self) -> &'t str {}
    #[inline]
    fn new(haystack: &'t str, start: usize, end: usize) -> Match<'t> {}
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
