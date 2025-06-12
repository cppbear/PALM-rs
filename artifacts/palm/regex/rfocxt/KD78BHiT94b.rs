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
    fn new() -> SingleByteSet {
        SingleByteSet {
            sparse: vec![false; 256],
            dense: vec![],
            complete: true,
            all_ascii: true,
        }
    }
    fn prefixes(lits: &Literals) -> SingleByteSet {}
    fn suffixes(lits: &Literals) -> SingleByteSet {}
    #[inline(always)]
    fn find(&self, text: &[u8]) -> Option<usize> {}
    fn _find(&self, haystack: &[u8]) -> Option<usize> {}
    fn approximate_size(&self) -> usize {}
}
