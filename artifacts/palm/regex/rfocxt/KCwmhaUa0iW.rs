use std::cmp;
use aho_corasick::{Automaton, AcAutomaton, FullAcAutomaton};
use syntax::hir::literal::Literals;
use vector::ssse3::{SSSE3VectorBuilder, u8x16};
const BLOCK_SIZE: usize = 16;
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
struct Masks {
    vb: AVX2VectorBuilder,
    masks: [Mask; 3],
    size: usize,
}
#[derive(Debug, Clone)]
struct Masks {
    vb: SSSE3VectorBuilder,
    masks: [Mask; 3],
    size: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct SSSE3VectorBuilder(());
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
