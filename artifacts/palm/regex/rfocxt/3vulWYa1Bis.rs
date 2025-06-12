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
pub struct Fsm<'a> {
    /// prog contains the NFA instruction opcodes. DFA execution uses either
    /// the `dfa` instructions or the `dfa_reverse` instructions from
    /// `exec::ExecReadOnly`. (It never uses `ExecReadOnly.nfa`, which may have
    /// Unicode opcodes that cannot be executed by the DFA.)
    prog: &'a Program,
    /// The start state. We record it here because the pointer may change
    /// when the cache is wiped.
    start: StatePtr,
    /// The current position in the input.
    at: usize,
    /// Should we quit after seeing the first match? e.g., When the caller
    /// uses `is_match` or `shortest_match`.
    quit_after_match: bool,
    /// The last state that matched.
    ///
    /// When no match has occurred, this is set to STATE_UNKNOWN.
    ///
    /// This is only useful when matching regex sets. The last match state
    /// is useful because it contains all of the match instructions seen,
    /// thereby allowing us to enumerate which regexes in the set matched.
    last_match_si: StatePtr,
    /// The input position of the last cache flush. We use this to determine
    /// if we're thrashing in the cache too often. If so, the DFA quits so
    /// that we can fall back to the NFA algorithm.
    last_cache_flush: usize,
    /// All cached DFA information that is persisted between searches.
    cache: &'a mut CacheInner,
}
#[derive(Clone, Debug)]
pub struct FreqyPacked {
    /// The pattern.
    pat: Vec<u8>,
    /// The number of Unicode characters in the pattern. This is useful for
    /// determining the effective length of a pattern when deciding which
    /// optimizations to perform. A trailing incomplete UTF-8 sequence counts
    /// as one character.
    char_len: usize,
    /// The rarest byte in the pattern, according to pre-computed frequency
    /// analysis.
    rare1: u8,
    /// The offset of the rarest byte in `pat`.
    rare1i: usize,
    /// The second rarest byte in the pattern, according to pre-computed
    /// frequency analysis. (This may be equivalent to the rarest byte.)
    ///
    /// The second rarest byte is used as a type of guard for quickly detecting
    /// a mismatch after memchr locates an instance of the rarest byte. This
    /// is a hedge against pathological cases where the pre-computed frequency
    /// analysis may be off. (But of course, does not prevent *all*
    /// pathological cases.)
    rare2: u8,
    /// The offset of the second rarest byte in `pat`.
    rare2i: usize,
}
#[derive(Clone, Debug)]
pub struct LiteralSearcher {
    complete: bool,
    lcp: FreqyPacked,
    lcs: FreqyPacked,
    matcher: Matcher,
}
#[derive(Clone)]
pub struct Program {
    /// A sequence of instructions that represents an NFA.
    pub insts: Vec<Inst>,
    /// Pointers to each Match instruction in the sequence.
    ///
    /// This is always length 1 unless this program represents a regex set.
    pub matches: Vec<InstPtr>,
    /// The ordered sequence of all capture groups extracted from the AST.
    /// Unnamed groups are `None`.
    pub captures: Vec<Option<String>>,
    /// Pointers to all named capture groups into `captures`.
    pub capture_name_idx: Arc<HashMap<String, usize>>,
    /// A pointer to the start instruction. This can vary depending on how
    /// the program was compiled. For example, programs for use with the DFA
    /// engine have a `.*?` inserted at the beginning of unanchored regular
    /// expressions. The actual starting point of the program is after the
    /// `.*?`.
    pub start: InstPtr,
    /// A set of equivalence classes for discriminating bytes in the compiled
    /// program.
    pub byte_classes: Vec<u8>,
    /// When true, this program can only match valid UTF-8.
    pub only_utf8: bool,
    /// When true, this program uses byte range instructions instead of Unicode
    /// range instructions.
    pub is_bytes: bool,
    /// When true, the program is compiled for DFA matching. For example, this
    /// implies `is_bytes` and also inserts a preceding `.*?` for unanchored
    /// regexes.
    pub is_dfa: bool,
    /// When true, the program matches text in reverse (for use only in the
    /// DFA).
    pub is_reverse: bool,
    /// Whether the regex must match from the start of the input.
    pub is_anchored_start: bool,
    /// Whether the regex must match at the end of the input.
    pub is_anchored_end: bool,
    /// Whether this program contains a Unicode word boundary instruction.
    pub has_unicode_word_boundary: bool,
    /// A possibly empty machine for very quickly matching prefix literals.
    pub prefixes: LiteralSearcher,
    /// A limit on the size of the cache that the DFA is allowed to use while
    /// matching.
    ///
    /// The cache limit specifies approximately how much space we're willing to
    /// give to the state cache. Once the state cache exceeds the size, it is
    /// wiped and all states must be re-computed.
    ///
    /// Note that this value does not impact correctness. It can be set to 0
    /// and the DFA will run just fine. (It will only ever store exactly one
    /// state in the cache, and will likely run very slowly, but it will work.)
    ///
    /// Also note that this limit is *per thread of execution*. That is,
    /// if the same regex is used to search text across multiple threads
    /// simultaneously, then the DFA cache is not shared. Instead, copies are
    /// made.
    pub dfa_size_limit: usize,
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
#[derive(Clone, Debug)]
pub struct ProgramCacheInner {
    pub pikevm: pikevm::Cache,
    pub backtrack: backtrack::Cache,
    pub dfa: dfa::Cache,
    pub dfa_reverse: dfa::Cache,
}
#[derive(Clone, Debug)]
pub enum Result<T> {
    Match(T),
    NoMatch(usize),
    Quit,
}
impl<'c> ExecNoSync<'c> {
    #[inline(always)]
    fn find_literals(
        &self,
        ty: MatchLiteralType,
        text: &[u8],
        start: usize,
    ) -> Option<(usize, usize)> {}
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
    ) -> Option<dfa::Result<(usize, usize)>> {
        use dfa::Result::*;
        let lcs = self.ro.suffixes.lcs();
        debug_assert!(lcs.len() >= 1);
        let mut start = original_start;
        let mut end = start;
        while end <= text.len() {
            start = end;
            end
                += match lcs.find(&text[end..]) {
                    None => return Some(NoMatch(text.len())),
                    Some(start) => start + lcs.len(),
                };
            match dfa::Fsm::reverse(
                &self.ro.dfa_reverse,
                self.cache,
                false,
                &text[start..end],
                end - start,
            ) {
                Match(0) | NoMatch(0) => return None,
                Match(s) => return Some(Match((s + start, end))),
                NoMatch(_) => continue,
                Quit => return Some(Quit),
            };
        }
        Some(NoMatch(text.len()))
    }
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
impl<'a> Fsm<'a> {
    #[inline(always)]
    pub fn forward(
        prog: &'a Program,
        cache: &ProgramCache,
        quit_after_match: bool,
        text: &[u8],
        at: usize,
    ) -> Result<usize> {}
    #[inline(always)]
    pub fn reverse(
        prog: &'a Program,
        cache: &ProgramCache,
        quit_after_match: bool,
        text: &[u8],
        at: usize,
    ) -> Result<usize> {
        let mut cache = cache.borrow_mut();
        let cache = &mut cache.dfa_reverse;
        let mut dfa = Fsm {
            prog: prog,
            start: 0,
            at: at,
            quit_after_match: quit_after_match,
            last_match_si: STATE_UNKNOWN,
            last_cache_flush: at,
            cache: &mut cache.inner,
        };
        let (empty_flags, state_flags) = dfa.start_flags_reverse(text, at);
        dfa.start = match dfa.start_state(&mut cache.qcur, empty_flags, state_flags) {
            None => return Result::Quit,
            Some(STATE_DEAD) => return Result::NoMatch(at),
            Some(si) => si,
        };
        debug_assert!(dfa.start != STATE_UNKNOWN);
        dfa.exec_at_reverse(&mut cache.qcur, &mut cache.qnext, text)
    }
    #[inline(always)]
    pub fn forward_many(
        prog: &'a Program,
        cache: &ProgramCache,
        matches: &mut [bool],
        text: &[u8],
        at: usize,
    ) -> Result<usize> {}
    #[inline(always)]
    fn exec_at(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        text: &[u8],
    ) -> Result<usize> {}
    #[inline(always)]
    fn exec_at_reverse(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        text: &[u8],
    ) -> Result<usize> {}
    #[inline(always)]
    unsafe fn next_si(&self, si: StatePtr, text: &[u8], i: usize) -> StatePtr {}
    fn exec_byte(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        mut si: StatePtr,
        b: Byte,
    ) -> Option<StatePtr> {}
    fn follow_epsilons(&mut self, ip: InstPtr, q: &mut SparseSet, flags: EmptyFlags) {}
    fn cached_state(
        &mut self,
        q: &SparseSet,
        mut state_flags: StateFlags,
        current_state: Option<&mut StatePtr>,
    ) -> Option<StatePtr> {}
    fn cached_state_key(
        &mut self,
        q: &SparseSet,
        state_flags: &mut StateFlags,
    ) -> Option<State> {}
    fn clear_cache_and_save(&mut self, current_state: Option<&mut StatePtr>) -> bool {}
    fn clear_cache(&mut self) -> bool {}
    fn restore_state(&mut self, state: State) -> Option<StatePtr> {}
    fn next_state(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        si: StatePtr,
        b: Byte,
    ) -> Option<StatePtr> {}
    #[inline(always)]
    fn start_state(
        &mut self,
        q: &mut SparseSet,
        empty_flags: EmptyFlags,
        state_flags: StateFlags,
    ) -> Option<StatePtr> {}
    fn start_flags(&self, text: &[u8], at: usize) -> (EmptyFlags, StateFlags) {}
    fn start_flags_reverse(&self, text: &[u8], at: usize) -> (EmptyFlags, StateFlags) {}
    fn state(&self, si: StatePtr) -> &State {}
    fn add_state(&mut self, state: State) -> Option<StatePtr> {}
    fn prefix_at(&self, text: &[u8], at: usize) -> Option<usize> {}
    fn num_byte_classes(&self) -> usize {}
    #[inline(always)]
    fn byte_class(&self, b: Byte) -> usize {}
    #[inline(always)]
    fn u8_class(&self, b: u8) -> usize {}
    fn continue_past_first_match(&self) -> bool {}
    fn has_prefix(&self) -> bool {}
    fn start_ptr(&self, si: StatePtr) -> StatePtr {}
    fn approximate_size(&self) -> usize {}
}
impl FreqyPacked {
    fn new(pat: Vec<u8>) -> FreqyPacked {}
    fn empty() -> FreqyPacked {}
    #[inline(always)]
    pub fn find(&self, haystack: &[u8]) -> Option<usize> {
        let pat = &*self.pat;
        if haystack.len() < pat.len() || pat.is_empty() {
            return None;
        }
        let mut i = self.rare1i;
        while i < haystack.len() {
            i
                += match memchr(self.rare1, &haystack[i..]) {
                    None => return None,
                    Some(i) => i,
                };
            let start = i - self.rare1i;
            let end = start + pat.len();
            if end > haystack.len() {
                return None;
            }
            let aligned = &haystack[start..end];
            if aligned[self.rare2i] == self.rare2 && aligned == &*self.pat {
                return Some(start);
            }
            i += 1;
        }
        None
    }
    #[inline(always)]
    pub fn is_suffix(&self, text: &[u8]) -> bool {}
    pub fn len(&self) -> usize {
        self.pat.len()
    }
    pub fn char_len(&self) -> usize {}
    fn approximate_size(&self) -> usize {}
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
    pub fn find(&self, haystack: &[u8]) -> Option<(usize, usize)> {}
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
    pub fn lcs(&self) -> &FreqyPacked {
        &self.lcs
    }
    pub fn is_empty(&self) -> bool {}
    pub fn len(&self) -> usize {}
    pub fn approximate_size(&self) -> usize {}
}
