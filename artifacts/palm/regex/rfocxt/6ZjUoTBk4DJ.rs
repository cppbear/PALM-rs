use std::cmp;
use std::mem;
use aho_corasick::{Automaton, AcAutomaton, FullAcAutomaton};
use memchr::{memchr, memchr2, memchr3};
use syntax::hir::literal::{Literal, Literals};
use freqs::BYTE_FREQUENCIES;
use self::teddy_avx2::Teddy as TeddyAVX2;
use self::teddy_ssse3::Teddy as TeddySSSE3;
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
impl FreqyPacked {
    fn new(pat: Vec<u8>) -> FreqyPacked {}
    fn empty() -> FreqyPacked {}
    #[inline(always)]
    pub fn find(&self, haystack: &[u8]) -> Option<usize> {}
    #[inline(always)]
    pub fn is_suffix(&self, text: &[u8]) -> bool {
        if text.len() < self.len() {
            return false;
        }
        text[text.len() - self.len()..] == *self.pat
    }
    pub fn len(&self) -> usize {
        self.pat.len()
    }
    pub fn char_len(&self) -> usize {}
    fn approximate_size(&self) -> usize {}
}
