use std::cmp;
use std::mem;
use aho_corasick::{Automaton, AcAutomaton, FullAcAutomaton};
use memchr::{memchr, memchr2, memchr3};
use syntax::hir::literal::{Literal, Literals};
use freqs::BYTE_FREQUENCIES;
use self::teddy_avx2::Teddy as TeddyAVX2;
use self::teddy_ssse3::Teddy as TeddySSSE3;
#[derive(Clone, Debug)]
struct SingleByteSet {
    sparse: Vec<bool>,
    dense: Vec<u8>,
    complete: bool,
    all_ascii: bool,
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
    fn _find(&self, haystack: &[u8]) -> Option<usize> {
        for (i, &b) in haystack.iter().enumerate() {
            if self.sparse[b as usize] {
                return Some(i);
            }
        }
        None
    }
    fn approximate_size(&self) -> usize {}
}
