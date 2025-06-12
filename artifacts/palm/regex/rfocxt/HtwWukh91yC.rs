pub type ProgramCache = RefCell<ProgramCacheInner>;
use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp;
use std::sync::Arc;
use thread_local::CachedThreadLocal;
use syntax::ParserBuilder;
use syntax::hir::Hir;
use syntax::hir::literal::Literals;
use backtrack;
use compile::Compiler;
use dfa;
use error::Error;
use input::{ByteInput, CharInput};
use literal::LiteralSearcher;
use pikevm;
use prog::Program;
use re_builder::RegexOptions;
use re_bytes;
use re_set;
use re_trait::{RegularExpression, Slot, Locations, as_slots};
use re_unicode;
use utf8::next_utf8;
pub trait RegularExpression: Sized {
    type Text: ?Sized;
    fn slots_len(&self) -> usize;
    fn locations(&self) -> Locations;
    fn next_after_empty(&self, text: &Self::Text, i: usize) -> usize;
    fn shortest_match_at(&self, text: &Self::Text, start: usize) -> Option<usize>;
    fn is_match_at(&self, text: &Self::Text, start: usize) -> bool;
    fn find_at(&self, text: &Self::Text, start: usize) -> Option<(usize, usize)>;
    fn read_captures_at(
        &self,
        locs: &mut Locations,
        text: &Self::Text,
        start: usize,
    ) -> Option<(usize, usize)>;
    fn find_iter(self, text: &Self::Text) -> Matches<Self> {
        Matches {
            re: self,
            text: text,
            last_end: 0,
            last_match: None,
        }
    }
    fn captures_iter(self, text: &Self::Text) -> CaptureMatches<Self> {
        CaptureMatches(self.find_iter(text))
    }
}
#[derive(Debug)]
pub struct ExecNoSync<'c> {
    /// All read only state.
    ro: &'c Arc<ExecReadOnly>,
    /// Caches for the various matching engines.
    cache: &'c ProgramCache,
}
#[derive(Clone, Debug)]
pub struct LiteralSearcher {
    complete: bool,
    lcp: FreqyPacked,
    lcs: FreqyPacked,
    matcher: Matcher,
}
#[derive(Debug)]
struct ExecReadOnly {
    /// The original regular expressions given by the caller to compile.
    res: Vec<String>,
    /// A compiled program that is used in the NFA simulation and backtracking.
    /// It can be byte-based or Unicode codepoint based.
    ///
    /// N.B. It is not possibly to make this byte-based from the public API.
    /// It is only used for testing byte based programs in the NFA simulations.
    nfa: Program,
    /// A compiled byte based program for DFA execution. This is only used
    /// if a DFA can be executed. (Currently, only word boundary assertions are
    /// not supported.) Note that this program contains an embedded `.*?`
    /// preceding the first capture group, unless the regex is anchored at the
    /// beginning.
    dfa: Program,
    /// The same as above, except the program is reversed (and there is no
    /// preceding `.*?`). This is used by the DFA to find the starting location
    /// of matches.
    dfa_reverse: Program,
    /// A set of suffix literals extracted from the regex.
    ///
    /// Prefix literals are stored on the `Program`, since they are used inside
    /// the matching engines.
    suffixes: LiteralSearcher,
    /// match_type encodes as much upfront knowledge about how we're going to
    /// execute a search as possible.
    match_type: MatchType,
}
#[derive(Clone, Copy, Debug)]
enum MatchLiteralType {
    /// Match literals anywhere in text.
    Unanchored,
    /// Match literals only at the start of text.
    AnchoredStart,
    /// Match literals only at the end of text.
    AnchoredEnd,
}
impl<'c> ExecNoSync<'c> {
    #[inline(always)]
    fn find_literals(
        &self,
        ty: MatchLiteralType,
        text: &[u8],
        start: usize,
    ) -> Option<(usize, usize)> {
        use self::MatchLiteralType::*;
        match ty {
            Unanchored => {
                let lits = &self.ro.nfa.prefixes;
                lits.find(&text[start..]).map(|(s, e)| (start + s, start + e))
            }
            AnchoredStart => {
                let lits = &self.ro.nfa.prefixes;
                if !self.ro.nfa.is_anchored_start
                    || (self.ro.nfa.is_anchored_start && start == 0)
                {
                    lits.find_start(&text[start..]).map(|(s, e)| (start + s, start + e))
                } else {
                    None
                }
            }
            AnchoredEnd => {
                let lits = &self.ro.suffixes;
                lits.find_end(&text[start..]).map(|(s, e)| (start + s, start + e))
            }
        }
    }
    #[inline(always)]
    fn find_dfa_forward(
        &self,
        text: &[u8],
        start: usize,
    ) -> dfa::Result<(usize, usize)> {}
    #[inline(always)]
    fn find_dfa_anchored_reverse(
        &self,
        text: &[u8],
        start: usize,
    ) -> dfa::Result<(usize, usize)> {}
    #[inline(always)]
    fn shortest_dfa(&self, text: &[u8], start: usize) -> dfa::Result<usize> {}
    #[inline(always)]
    fn shortest_dfa_reverse_suffix(
        &self,
        text: &[u8],
        start: usize,
    ) -> dfa::Result<usize> {}
    #[inline(always)]
    fn exec_dfa_reverse_suffix(
        &self,
        text: &[u8],
        original_start: usize,
    ) -> Option<dfa::Result<(usize, usize)>> {}
    #[inline(always)]
    fn find_dfa_reverse_suffix(
        &self,
        text: &[u8],
        start: usize,
    ) -> dfa::Result<(usize, usize)> {}
    fn match_nfa(&self, text: &[u8], start: usize) -> bool {}
    fn match_nfa_type(&self, ty: MatchNfaType, text: &[u8], start: usize) -> bool {}
    fn shortest_nfa(&self, text: &[u8], start: usize) -> Option<usize> {}
    fn shortest_nfa_type(
        &self,
        ty: MatchNfaType,
        text: &[u8],
        start: usize,
    ) -> Option<usize> {}
    fn find_nfa(
        &self,
        ty: MatchNfaType,
        text: &[u8],
        start: usize,
    ) -> Option<(usize, usize)> {}
    fn captures_nfa_with_match(
        &self,
        slots: &mut [Slot],
        text: &[u8],
        match_start: usize,
        match_end: usize,
    ) -> Option<(usize, usize)> {}
    fn captures_nfa(
        &self,
        slots: &mut [Slot],
        text: &[u8],
        start: usize,
    ) -> Option<(usize, usize)> {}
    fn captures_nfa_type(
        &self,
        ty: MatchNfaType,
        slots: &mut [Slot],
        text: &[u8],
        start: usize,
    ) -> Option<(usize, usize)> {}
    fn exec_nfa(
        &self,
        mut ty: MatchNfaType,
        matches: &mut [bool],
        slots: &mut [Slot],
        quit_after_match: bool,
        text: &[u8],
        start: usize,
    ) -> bool {}
    fn exec_pikevm(
        &self,
        matches: &mut [bool],
        slots: &mut [Slot],
        quit_after_match: bool,
        text: &[u8],
        start: usize,
    ) -> bool {}
    fn exec_backtrack(
        &self,
        matches: &mut [bool],
        slots: &mut [Slot],
        text: &[u8],
        start: usize,
    ) -> bool {}
    pub fn many_matches_at(
        &self,
        matches: &mut [bool],
        text: &[u8],
        start: usize,
    ) -> bool {}
    #[inline(always)]
    fn is_anchor_end_match(&self, text: &[u8]) -> bool {}
    pub fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {}
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
    pub fn find_start(&self, haystack: &[u8]) -> Option<(usize, usize)> {
        for lit in self.iter() {
            if lit.len() > haystack.len() {
                continue;
            }
            if lit == &haystack[0..lit.len()] {
                return Some((0, lit.len()));
            }
        }
        None
    }
    pub fn find_end(&self, haystack: &[u8]) -> Option<(usize, usize)> {
        for lit in self.iter() {
            if lit.len() > haystack.len() {
                continue;
            }
            if lit == &haystack[haystack.len() - lit.len()..] {
                return Some((haystack.len() - lit.len(), haystack.len()));
            }
        }
        None
    }
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
