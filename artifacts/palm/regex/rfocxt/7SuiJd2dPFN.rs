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
impl<'c> RegularExpression for ExecNoSync<'c> {
    type Text = [u8];
    fn slots_len(&self) -> usize {}
    fn next_after_empty(&self, _text: &[u8], i: usize) -> usize {
        i + 1
    }
    #[inline(always)]
    fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {}
    #[inline(always)]
    fn is_match_at(&self, text: &[u8], start: usize) -> bool {}
    #[inline(always)]
    fn find_at(&self, text: &[u8], start: usize) -> Option<(usize, usize)> {}
    fn read_captures_at(
        &self,
        locs: &mut Locations,
        text: &[u8],
        start: usize,
    ) -> Option<(usize, usize)> {}
}
