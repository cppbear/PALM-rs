use std::cmp;
use std::mem;
use aho_corasick::{Automaton, AcAutomaton, FullAcAutomaton};
use memchr::{memchr, memchr2, memchr3};
use syntax::hir::literal::{Literal, Literals};
use freqs::BYTE_FREQUENCIES;
use self::teddy_avx2::Teddy as TeddyAVX2;
use self::teddy_ssse3::Teddy as TeddySSSE3;
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
#[derive(Clone, Debug)]
struct SingleByteSet {
    sparse: Vec<bool>,
    dense: Vec<u8>,
    complete: bool,
    all_ascii: bool,
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
impl Matcher {
    fn prefixes(lits: &Literals) -> Self {
        let sset = SingleByteSet::prefixes(lits);
        Matcher::new(lits, sset)
    }
    fn suffixes(lits: &Literals) -> Self {
        let sset = SingleByteSet::suffixes(lits);
        Matcher::new(lits, sset)
    }
    fn new(lits: &Literals, sset: SingleByteSet) -> Self {
        if lits.literals().is_empty() {
            return Matcher::Empty;
        }
        if sset.dense.len() >= 26 {
            return Matcher::Empty;
        }
        if sset.complete {
            return Matcher::Bytes(sset);
        }
        if lits.literals().len() == 1 {
            let lit = lits.literals()[0].to_vec();
            if BoyerMooreSearch::should_use(lit.as_slice()) {
                return Matcher::BoyerMoore(BoyerMooreSearch::new(lit));
            } else {
                return Matcher::FreqyPacked(FreqyPacked::new(lit));
            }
        }
        let is_aho_corasick_fast = sset.dense.len() == 1 && sset.all_ascii;
        if TeddyAVX2::available() && !is_aho_corasick_fast {
            const MAX_TEDDY_LITERALS: usize = 32;
            if lits.literals().len() <= MAX_TEDDY_LITERALS {
                if let Some(ted) = TeddyAVX2::new(lits) {
                    return Matcher::TeddyAVX2(ted);
                }
            }
        }
        if TeddySSSE3::available() && !is_aho_corasick_fast {
            const MAX_TEDDY_LITERALS: usize = 32;
            if lits.literals().len() <= MAX_TEDDY_LITERALS {
                if let Some(ted) = TeddySSSE3::new(lits) {
                    return Matcher::TeddySSSE3(ted);
                }
            }
        }
        let pats = lits.literals().to_owned();
        Matcher::AC(AcAutomaton::new(pats).into_full())
    }
}
impl Teddy {
    pub fn available() -> bool {
        SSSE3VectorBuilder::new().is_some()
    }
    pub fn new(pats: &Literals) -> Option<Teddy> {
        let vb = match SSSE3VectorBuilder::new() {
            None => return None,
            Some(vb) => vb,
        };
        if !Teddy::available() {
            return None;
        }
        let pats: Vec<_> = pats.literals().iter().map(|p| p.to_vec()).collect();
        let min_len = pats.iter().map(|p| p.len()).min().unwrap_or(0);
        if min_len < 1 {
            return None;
        }
        let nmasks = cmp::min(3, min_len);
        let mut masks = Masks::new(vb, nmasks);
        let mut buckets = vec![vec![]; 8];
        for (pati, pat) in pats.iter().enumerate() {
            let bucket = pati % 8;
            buckets[bucket].push(pati);
            masks.add(bucket as u8, pat);
        }
        Some(Teddy {
            vb: vb,
            pats: pats.to_vec(),
            ac: AcAutomaton::new(pats.to_vec()).into_full(),
            buckets: buckets,
            masks: masks,
        })
    }
    pub fn patterns(&self) -> &[Vec<u8>] {}
    pub fn len(&self) -> usize {}
    pub fn approximate_size(&self) -> usize {}
    pub fn find(&self, haystack: &[u8]) -> Option<Match> {}
    #[allow(unused_attributes)]
    #[target_feature(enable = "avx2")]
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
        res: u8x32,
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
impl FreqyPacked {
    fn new(pat: Vec<u8>) -> FreqyPacked {
        if pat.is_empty() {
            return FreqyPacked::empty();
        }
        let mut rare1 = pat[0];
        let mut rare2 = pat[0];
        for b in pat[1..].iter().cloned() {
            if freq_rank(b) < freq_rank(rare1) {
                rare1 = b;
            }
        }
        for &b in &pat {
            if rare1 == rare2 {
                rare2 = b
            } else if b != rare1 && freq_rank(b) < freq_rank(rare2) {
                rare2 = b;
            }
        }
        let rare1i = pat.iter().rposition(|&b| b == rare1).unwrap();
        let rare2i = pat.iter().rposition(|&b| b == rare2).unwrap();
        let char_len = char_len_lossy(&pat);
        FreqyPacked {
            pat: pat,
            char_len: char_len,
            rare1: rare1,
            rare1i: rare1i,
            rare2: rare2,
            rare2i: rare2i,
        }
    }
    fn empty() -> FreqyPacked {}
    #[inline(always)]
    pub fn find(&self, haystack: &[u8]) -> Option<usize> {}
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
    fn find(&self, haystack: &[u8]) -> Option<usize> {}
    fn len(&self) -> usize {}
    fn should_use(pattern: &[u8]) -> bool {
        const MIN_LEN: usize = 9;
        const MIN_CUTOFF: usize = 150;
        const MAX_CUTOFF: usize = 255;
        const LEN_CUTOFF_PROPORTION: usize = 4;
        let scaled_rank = pattern.len().wrapping_mul(LEN_CUTOFF_PROPORTION);
        let cutoff = cmp::max(
            MIN_CUTOFF,
            MAX_CUTOFF - cmp::min(MAX_CUTOFF, scaled_rank),
        );
        pattern.len() > MIN_LEN && pattern.iter().all(|c| freq_rank(*c) >= cutoff)
    }
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
