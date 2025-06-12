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
impl<'t> Captures<'t> {
    pub fn get(&self, i: usize) -> Option<Match<'t>> {}
    pub fn name(&self, name: &str) -> Option<Match<'t>> {}
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 't> {}
    pub fn expand(&self, replacement: &str, dst: &mut String) {
        expand_str(self, replacement, dst)
    }
    #[inline]
    pub fn len(&self) -> usize {}
}
pub fn expand_str(caps: &re_unicode::Captures, mut replacement: &str, dst: &mut String) {
    while !replacement.is_empty() {
        match memchr(b'$', replacement.as_bytes()) {
            None => break,
            Some(i) => {
                dst.push_str(&replacement[..i]);
                replacement = &replacement[i..];
            }
        }
        if replacement.as_bytes().get(1).map_or(false, |&b| b == b'$') {
            dst.push_str("$");
            replacement = &replacement[2..];
            continue;
        }
        debug_assert!(! replacement.is_empty());
        let cap_ref = match find_cap_ref(replacement) {
            Some(cap_ref) => cap_ref,
            None => {
                dst.push_str("$");
                replacement = &replacement[1..];
                continue;
            }
        };
        replacement = &replacement[cap_ref.end..];
        match cap_ref.cap {
            Ref::Number(i) => {
                dst.push_str(caps.get(i).map(|m| m.as_str()).unwrap_or(""));
            }
            Ref::Named(name) => {
                dst.push_str(caps.name(name).map(|m| m.as_str()).unwrap_or(""));
            }
        }
    }
    dst.push_str(replacement);
}
