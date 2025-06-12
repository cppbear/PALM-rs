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
impl SingleByteSet {
    fn new() -> SingleByteSet {}
    fn prefixes(lits: &Literals) -> SingleByteSet {
        let mut sset = SingleByteSet::new();
        for lit in lits.literals() {
            sset.complete = sset.complete && lit.len() == 1;
            if let Some(&b) = lit.get(0) {
                if !sset.sparse[b as usize] {
                    if b > 0x7F {
                        sset.all_ascii = false;
                    }
                    sset.dense.push(b);
                    sset.sparse[b as usize] = true;
                }
            }
        }
        sset
    }
    fn suffixes(lits: &Literals) -> SingleByteSet {}
    #[inline(always)]
    fn find(&self, text: &[u8]) -> Option<usize> {}
    fn _find(&self, haystack: &[u8]) -> Option<usize> {}
    fn approximate_size(&self) -> usize {}
}
