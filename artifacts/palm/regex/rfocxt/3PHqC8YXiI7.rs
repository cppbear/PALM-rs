use std::cmp;
use std::mem;
use aho_corasick::{Automaton, AcAutomaton, FullAcAutomaton};
use memchr::{memchr, memchr2, memchr3};
use syntax::hir::literal::{Literal, Literals};
use freqs::BYTE_FREQUENCIES;
use self::teddy_avx2::Teddy as TeddyAVX2;
use self::teddy_ssse3::Teddy as TeddySSSE3;
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
    fn len(&self) -> usize {
        return self.pattern.len();
    }
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
