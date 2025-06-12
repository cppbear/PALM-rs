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
pub enum LiteralIter<'a> {
    Empty,
    Bytes(&'a [u8]),
    Single(&'a [u8]),
    AC(&'a [Literal]),
    TeddySSSE3(&'a [Vec<u8>]),
    TeddyAVX2(&'a [Vec<u8>]),
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
    pub fn lcs(&self) -> &FreqyPacked {}
    pub fn is_empty(&self) -> bool {}
    pub fn len(&self) -> usize {}
    pub fn approximate_size(&self) -> usize {}
}
impl Teddy {
    pub fn available() -> bool {}
    pub fn new(pats: &Literals) -> Option<Teddy> {}
    pub fn patterns(&self) -> &[Vec<u8>] {
        &self.pats
    }
    pub fn len(&self) -> usize {}
    pub fn approximate_size(&self) -> usize {}
    pub fn find(&self, haystack: &[u8]) -> Option<Match> {}
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
