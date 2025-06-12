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
    pub fn new(re: &str) -> Result<Regex, Error> {}
    pub fn is_match(&self, text: &str) -> bool {}
    pub fn find<'t>(&self, text: &'t str) -> Option<Match<'t>> {}
    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't> {}
    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {}
    pub fn captures_iter<'r, 't>(&'r self, text: &'t str) -> CaptureMatches<'r, 't> {}
    pub fn split<'r, 't>(&'r self, text: &'t str) -> Split<'r, 't> {}
    pub fn splitn<'r, 't>(&'r self, text: &'t str, limit: usize) -> SplitN<'r, 't> {}
    pub fn replace<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {}
    pub fn replace_all<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {
        self.replacen(text, 0, rep)
    }
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
