use std::char;
use std::cmp::Ordering;
use std::fmt;
use std::ops;
use std::u32;
use syntax;
use literal::LiteralSearcher;
use prog::InstEmptyLook;
use utf8::{decode_utf8, decode_last_utf8};
pub trait Input {
    fn at(&self, i: usize) -> InputAt;
    fn next_char(&self, at: InputAt) -> Char;
    fn previous_char(&self, at: InputAt) -> Char;
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool;
    fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn as_bytes(&self) -> &[u8];
}
#[derive(Clone, Copy, Debug)]
pub struct ByteInput<'t> {
    text: &'t [u8],
    only_utf8: bool,
}
#[derive(Clone, Debug)]
pub struct LiteralSearcher {
    complete: bool,
    lcp: FreqyPacked,
    lcs: FreqyPacked,
    matcher: Matcher,
}
#[derive(Clone, Copy, Debug)]
pub struct InputAt {
    pos: usize,
    c: Char,
    byte: Option<u8>,
    len: usize,
}
impl<'t> Input for ByteInput<'t> {
    fn at(&self, i: usize) -> InputAt {}
    fn next_char(&self, at: InputAt) -> Char {}
    fn previous_char(&self, at: InputAt) -> Char {}
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {}
    fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {
        prefixes.find(&self[at.pos()..]).map(|(s, _)| self.at(at.pos() + s))
    }
    fn len(&self) -> usize {}
    fn as_bytes(&self) -> &[u8] {}
}
impl LiteralSearcher {
    pub fn empty() -> Self {
        Self::new(Literals::empty(), Matcher::Empty)
    }
    pub fn prefixes(lits: Literals) -> Self {
        let matcher = Matcher::prefixes(&lits);
        Self::new(lits, matcher)
    }
    pub fn suffixes(lits: Literals) -> Self {
        let matcher = Matcher::suffixes(&lits);
        Self::new(lits, matcher)
    }
    fn new(lits: Literals, matcher: Matcher) -> Self {
        let complete = lits.all_complete();
        LiteralSearcher {
            complete: complete,
            lcp: FreqyPacked::new(lits.longest_common_prefix().to_vec()),
            lcs: FreqyPacked::new(lits.longest_common_suffix().to_vec()),
            matcher: matcher,
        }
    }
    pub fn complete(&self) -> bool {}
    #[inline(always)]
    pub fn find(&self, haystack: &[u8]) -> Option<(usize, usize)> {
        use self::Matcher::*;
        match self.matcher {
            Empty => Some((0, 0)),
            Bytes(ref sset) => sset.find(haystack).map(|i| (i, i + 1)),
            FreqyPacked(ref s) => s.find(haystack).map(|i| (i, i + s.len())),
            BoyerMoore(ref s) => s.find(haystack).map(|i| (i, i + s.len())),
            AC(ref aut) => aut.find(haystack).next().map(|m| (m.start, m.end)),
            TeddySSSE3(ref t) => t.find(haystack).map(|m| (m.start, m.end)),
            TeddyAVX2(ref t) => t.find(haystack).map(|m| (m.start, m.end)),
        }
    }
    pub fn find_start(&self, haystack: &[u8]) -> Option<(usize, usize)> {}
    pub fn find_end(&self, haystack: &[u8]) -> Option<(usize, usize)> {}
    pub fn iter(&self) -> LiteralIter {
        match self.matcher {
            Matcher::Empty => LiteralIter::Empty,
            Matcher::Bytes(ref sset) => LiteralIter::Bytes(&sset.dense),
            Matcher::FreqyPacked(ref s) => LiteralIter::Single(&s.pat),
            Matcher::BoyerMoore(ref s) => LiteralIter::Single(&s.pattern),
            Matcher::AC(ref ac) => LiteralIter::AC(ac.patterns()),
            Matcher::TeddySSSE3(ref ted) => LiteralIter::TeddySSSE3(ted.patterns()),
            Matcher::TeddyAVX2(ref ted) => LiteralIter::TeddyAVX2(ted.patterns()),
        }
    }
    pub fn lcp(&self) -> &FreqyPacked {}
    pub fn lcs(&self) -> &FreqyPacked {}
    pub fn is_empty(&self) -> bool {}
    pub fn len(&self) -> usize {}
    pub fn approximate_size(&self) -> usize {}
}
impl InputAt {
    pub fn is_start(&self) -> bool {}
    pub fn is_end(&self) -> bool {}
    pub fn char(&self) -> Char {}
    pub fn byte(&self) -> Option<u8> {}
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn pos(&self) -> usize {
        self.pos
    }
    pub fn next_pos(&self) -> usize {}
}
