use std::cmp;
use std::mem;
use aho_corasick::{Automaton, AcAutomaton, FullAcAutomaton};
use memchr::{memchr, memchr2, memchr3};
use syntax::hir::literal::{Literal, Literals};
use freqs::BYTE_FREQUENCIES;
use self::teddy_avx2::Teddy as TeddyAVX2;
use self::teddy_ssse3::Teddy as TeddySSSE3;
#[derive(Clone, Debug)]
pub struct LiteralSearcher {
    complete: bool,
    lcp: FreqyPacked,
    lcs: FreqyPacked,
    matcher: Matcher,
}
#[derive(Debug, Clone)]
pub struct Teddy {
    /// A builder for SSSE3 empowered vectors.
    vb: SSSE3VectorBuilder,
    /// A list of substrings to match.
    pats: Vec<Vec<u8>>,
    /// An Aho-Corasick automaton of the patterns. We use this when we need to
    /// search pieces smaller than the Teddy block size.
    ac: FullAcAutomaton<Vec<u8>>,
    /// A set of 8 buckets. Each bucket corresponds to a single member of a
    /// bitset. A bucket contains zero or more substrings. This is useful
    /// when the number of substrings exceeds 8, since our bitsets cannot have
    /// more than 8 members.
    buckets: Vec<Vec<usize>>,
    /// Our set of masks. There's one mask for each byte in the fingerprint.
    masks: Masks,
}
#[derive(Debug, Clone)]
pub struct Teddy {
    /// A builder for AVX2 empowered vectors.
    vb: AVX2VectorBuilder,
    /// A list of substrings to match.
    pats: Vec<Vec<u8>>,
    /// An Aho-Corasick automaton of the patterns. We use this when we need to
    /// search pieces smaller than the Teddy block size.
    ac: FullAcAutomaton<Vec<u8>>,
    /// A set of 8 buckets. Each bucket corresponds to a single member of a
    /// bitset. A bucket contains zero or more substrings. This is useful
    /// when the number of substrings exceeds 8, since our bitsets cannot have
    /// more than 8 members.
    buckets: Vec<Vec<usize>>,
    /// Our set of masks. There's one mask for each byte in the fingerprint.
    masks: Masks,
}
#[derive(Clone, Debug)]
struct SingleByteSet {
    sparse: Vec<bool>,
    dense: Vec<u8>,
    complete: bool,
    all_ascii: bool,
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
pub struct BoyerMooreSearch {
    /// The pattern we are going to look for in the haystack.
    pattern: Vec<u8>,
    /// The skip table for the skip loop.
    ///
    /// Maps the character at the end of the input
    /// to a shift.
    skip_table: Vec<usize>,
    /// The guard character (least frequently occurring char).
    guard: u8,
    /// The reverse-index of the guard character in the pattern.
    guard_reverse_idx: usize,
    /// Daniel Sunday's mini generalized delta2 shift table.
    ///
    /// We use a skip loop, so we only have to provide a shift
    /// for the skip char (last char). This is why it is a mini
    /// shift rule.
    md2_shift: usize,
}
#[derive(Debug, Clone)]
pub struct Match {
    /// The index of the pattern that matched. The index is in correspondence
    /// with the order of the patterns given at construction.
    pub pat: usize,
    /// The start byte offset of the match.
    pub start: usize,
    /// The end byte offset of the match. This is always `start + pat.len()`.
    pub end: usize,
}
#[derive(Debug, Clone)]
pub struct Match {
    /// The index of the pattern that matched. The index is in correspondence
    /// with the order of the patterns given at construction.
    pub pat: usize,
    /// The start byte offset of the match.
    pub start: usize,
    /// The end byte offset of the match. This is always `start + pat.len()`.
    pub end: usize,
}
#[derive(Clone, Debug)]
enum Matcher {
    /// No literals. (Never advances through the input.)
    Empty,
    /// A set of four or more single byte literals.
    Bytes(SingleByteSet),
    /// A single substring, find using memchr and frequency analysis.
    FreqyPacked(FreqyPacked),
    /// A single substring, find using Boyer-Moore.
    BoyerMoore(BoyerMooreSearch),
    /// An Aho-Corasick automaton.
    AC(FullAcAutomaton<Literal>),
    /// A simd accelerated multiple string matcher. Used only for a small
    /// number of small literals.
    TeddySSSE3(TeddySSSE3),
    /// A simd accelerated multiple string matcher. Used only for a small
    /// number of small literals. This uses 256-bit vectors.
    TeddyAVX2(TeddyAVX2),
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
impl Teddy {
    pub fn available() -> bool {}
    pub fn new(pats: &Literals) -> Option<Teddy> {}
    pub fn patterns(&self) -> &[Vec<u8>] {}
    pub fn len(&self) -> usize {}
    pub fn approximate_size(&self) -> usize {}
    pub fn find(&self, haystack: &[u8]) -> Option<Match> {
        unsafe { self.find_impl(haystack) }
    }
    #[allow(unused_attributes)]
    #[target_feature(enable = "ssse3")]
    unsafe fn find_impl(&self, haystack: &[u8]) -> Option<Match> {}
    #[inline(always)]
    fn find1(&self, haystack: &[u8]) -> Option<Match> {}
    #[inline(always)]
    fn find2(&self, haystack: &[u8]) -> Option<Match> {}
    #[inline(always)]
    fn find3(&self, haystack: &[u8]) -> Option<Match> {}
    #[inline(always)]
    fn verify(
        &self,
        haystack: &[u8],
        pos: usize,
        res: u8x16,
        mut bitfield: u32,
    ) -> Option<Match> {}
    #[inline(always)]
    fn verify_bucket(
        &self,
        haystack: &[u8],
        bucket: usize,
        start: usize,
    ) -> Option<Match> {}
    #[inline(never)]
    fn slow(&self, haystack: &[u8], pos: usize) -> Option<Match> {}
}
impl SingleByteSet {
    fn new() -> SingleByteSet {}
    fn prefixes(lits: &Literals) -> SingleByteSet {}
    fn suffixes(lits: &Literals) -> SingleByteSet {}
    #[inline(always)]
    fn find(&self, text: &[u8]) -> Option<usize> {
        match self.dense.len() {
            0 => None,
            1 => memchr(self.dense[0], text),
            2 => memchr2(self.dense[0], self.dense[1], text),
            3 => memchr3(self.dense[0], self.dense[1], self.dense[2], text),
            _ => self._find(text),
        }
    }
    fn _find(&self, haystack: &[u8]) -> Option<usize> {}
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
    pub fn len(&self) -> usize {}
    pub fn char_len(&self) -> usize {}
    fn approximate_size(&self) -> usize {}
}
impl BoyerMooreSearch {
    fn new(pattern: Vec<u8>) -> Self {
        debug_assert!(pattern.len() > 0);
        let (g, gi) = Self::select_guard(pattern.as_slice());
        let skip_table = Self::compile_skip_table(pattern.as_slice());
        let md2_shift = Self::compile_md2_shift(pattern.as_slice());
        BoyerMooreSearch {
            pattern: pattern,
            skip_table: skip_table,
            guard: g,
            guard_reverse_idx: gi,
            md2_shift: md2_shift,
        }
    }
    #[inline]
    fn find(&self, haystack: &[u8]) -> Option<usize> {
        if haystack.len() < self.pattern.len() {
            return None;
        }
        let mut window_end = self.pattern.len() - 1;
        const NUM_UNROLL: usize = 10;
        let short_circut = (NUM_UNROLL + 2) * self.pattern.len();
        if haystack.len() > short_circut {
            let backstop = haystack.len() - ((NUM_UNROLL + 1) * self.pattern.len());
            loop {
                window_end = match self.skip_loop(haystack, window_end, backstop) {
                    Some(i) => i,
                    None => return None,
                };
                if window_end >= backstop {
                    break;
                }
                if self.check_match(haystack, window_end) {
                    return Some(window_end - (self.pattern.len() - 1));
                } else {
                    let skip = self.skip_table[haystack[window_end] as usize];
                    window_end += if skip == 0 { self.md2_shift } else { skip };
                    continue;
                }
            }
        }
        while window_end < haystack.len() {
            let mut skip = self.skip_table[haystack[window_end] as usize];
            if skip == 0 {
                if self.check_match(haystack, window_end) {
                    return Some(window_end - (self.pattern.len() - 1));
                } else {
                    skip = self.md2_shift;
                }
            }
            window_end += skip;
        }
        None
    }
    fn len(&self) -> usize {}
    fn should_use(pattern: &[u8]) -> bool {}
    #[inline]
    fn check_match(&self, haystack: &[u8], window_end: usize) -> bool {}
    #[inline]
    fn skip_loop(
        &self,
        haystack: &[u8],
        mut window_end: usize,
        backstop: usize,
    ) -> Option<usize> {}
    fn compile_skip_table(pattern: &[u8]) -> Vec<usize> {}
    fn select_guard(pattern: &[u8]) -> (u8, usize) {}
    fn compile_md2_shift(pattern: &[u8]) -> usize {}
    fn approximate_size(&self) -> usize {}
}
