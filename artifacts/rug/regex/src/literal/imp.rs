use std::mem;

use aho_corasick::{self, packed, AhoCorasick};
use memchr::{memchr, memchr2, memchr3, memmem};
use regex_syntax::hir::literal::{Literal, Seq};

/// A prefix extracted from a compiled regular expression.
///
/// A regex prefix is a set of literal strings that *must* be matched at the
/// beginning of a regex in order for the entire regex to match. Similarly
/// for a regex suffix.
#[derive(Clone, Debug)]
pub struct LiteralSearcher {
    complete: bool,
    lcp: Memmem,
    lcs: Memmem,
    matcher: Matcher,
}

#[derive(Clone, Debug)]
enum Matcher {
    /// No literals. (Never advances through the input.)
    Empty,
    /// A set of four or more single byte literals.
    Bytes(SingleByteSet),
    /// A single substring, using vector accelerated routines when available.
    Memmem(Memmem),
    /// An Aho-Corasick automaton.
    AC { ac: AhoCorasick, lits: Vec<Literal> },
    /// A packed multiple substring searcher, using SIMD.
    ///
    /// Note that Aho-Corasick will actually use this packed searcher
    /// internally automatically, however, there is some overhead associated
    /// with going through the Aho-Corasick machinery. So using the packed
    /// searcher directly results in some gains.
    Packed { s: packed::Searcher, lits: Vec<Literal> },
}

impl LiteralSearcher {
    /// Returns a matcher that never matches and never advances the input.
    pub fn empty() -> Self {
        Self::new(Seq::infinite(), Matcher::Empty)
    }

    /// Returns a matcher for literal prefixes from the given set.
    pub fn prefixes(lits: Seq) -> Self {
        let matcher = Matcher::prefixes(&lits);
        Self::new(lits, matcher)
    }

    /// Returns a matcher for literal suffixes from the given set.
    pub fn suffixes(lits: Seq) -> Self {
        let matcher = Matcher::suffixes(&lits);
        Self::new(lits, matcher)
    }

    fn new(lits: Seq, matcher: Matcher) -> Self {
        LiteralSearcher {
            complete: lits.is_exact(),
            lcp: Memmem::new(lits.longest_common_prefix().unwrap_or(b"")),
            lcs: Memmem::new(lits.longest_common_suffix().unwrap_or(b"")),
            matcher,
        }
    }

    /// Returns true if all matches comprise the entire regular expression.
    ///
    /// This does not necessarily mean that a literal match implies a match
    /// of the regular expression. For example, the regular expression `^a`
    /// is comprised of a single complete literal `a`, but the regular
    /// expression demands that it only match at the beginning of a string.
    pub fn complete(&self) -> bool {
        self.complete && !self.is_empty()
    }

    /// Find the position of a literal in `haystack` if it exists.
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn find(&self, haystack: &[u8]) -> Option<(usize, usize)> {
        use self::Matcher::*;
        match self.matcher {
            Empty => Some((0, 0)),
            Bytes(ref sset) => sset.find(haystack).map(|i| (i, i + 1)),
            Memmem(ref s) => s.find(haystack).map(|i| (i, i + s.len())),
            AC { ref ac, .. } => {
                ac.find(haystack).map(|m| (m.start(), m.end()))
            }
            Packed { ref s, .. } => {
                s.find(haystack).map(|m| (m.start(), m.end()))
            }
        }
    }

    /// Like find, except matches must start at index `0`.
    pub fn find_start(&self, haystack: &[u8]) -> Option<(usize, usize)> {
        for lit in self.iter() {
            if lit.len() > haystack.len() {
                continue;
            }
            if lit == &haystack[0..lit.len()] {
                return Some((0, lit.len()));
            }
        }
        None
    }

    /// Like find, except matches must end at index `haystack.len()`.
    pub fn find_end(&self, haystack: &[u8]) -> Option<(usize, usize)> {
        for lit in self.iter() {
            if lit.len() > haystack.len() {
                continue;
            }
            if lit == &haystack[haystack.len() - lit.len()..] {
                return Some((haystack.len() - lit.len(), haystack.len()));
            }
        }
        None
    }

    /// Returns an iterator over all literals to be matched.
    pub fn iter(&self) -> LiteralIter<'_> {
        match self.matcher {
            Matcher::Empty => LiteralIter::Empty,
            Matcher::Bytes(ref sset) => LiteralIter::Bytes(&sset.dense),
            Matcher::Memmem(ref s) => LiteralIter::Single(&s.finder.needle()),
            Matcher::AC { ref lits, .. } => LiteralIter::AC(lits),
            Matcher::Packed { ref lits, .. } => LiteralIter::Packed(lits),
        }
    }

    /// Returns a matcher for the longest common prefix of this matcher.
    pub fn lcp(&self) -> &Memmem {
        &self.lcp
    }

    /// Returns a matcher for the longest common suffix of this matcher.
    pub fn lcs(&self) -> &Memmem {
        &self.lcs
    }

    /// Returns true iff this prefix is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of prefixes in this machine.
    pub fn len(&self) -> usize {
        use self::Matcher::*;
        match self.matcher {
            Empty => 0,
            Bytes(ref sset) => sset.dense.len(),
            Memmem(_) => 1,
            AC { ref ac, .. } => ac.patterns_len(),
            Packed { ref lits, .. } => lits.len(),
        }
    }

    /// Return the approximate heap usage of literals in bytes.
    pub fn approximate_size(&self) -> usize {
        use self::Matcher::*;
        match self.matcher {
            Empty => 0,
            Bytes(ref sset) => sset.approximate_size(),
            Memmem(ref single) => single.approximate_size(),
            AC { ref ac, .. } => ac.memory_usage(),
            Packed { ref s, .. } => s.memory_usage(),
        }
    }
}

impl Matcher {
    fn prefixes(lits: &Seq) -> Self {
        let sset = SingleByteSet::prefixes(lits);
        Matcher::new(lits, sset)
    }

    fn suffixes(lits: &Seq) -> Self {
        let sset = SingleByteSet::suffixes(lits);
        Matcher::new(lits, sset)
    }

    fn new(lits: &Seq, sset: SingleByteSet) -> Self {
        if lits.is_empty() || lits.min_literal_len() == Some(0) {
            return Matcher::Empty;
        }
        let lits = match lits.literals() {
            None => return Matcher::Empty,
            Some(members) => members,
        };
        if sset.dense.len() >= 26 {
            // Avoid trying to match a large number of single bytes.
            // This is *very* sensitive to a frequency analysis comparison
            // between the bytes in sset and the composition of the haystack.
            // No matter the size of sset, if its members all are rare in the
            // haystack, then it'd be worth using it. How to tune this... IDK.
            // ---AG
            return Matcher::Empty;
        }
        if sset.complete {
            return Matcher::Bytes(sset);
        }
        if lits.len() == 1 {
            return Matcher::Memmem(Memmem::new(lits[0].as_bytes()));
        }

        let pats: Vec<&[u8]> = lits.iter().map(|lit| lit.as_bytes()).collect();
        let is_aho_corasick_fast = sset.dense.len() <= 1 && sset.all_ascii;
        if lits.len() <= 100 && !is_aho_corasick_fast {
            let mut builder = packed::Config::new()
                .match_kind(packed::MatchKind::LeftmostFirst)
                .builder();
            if let Some(s) = builder.extend(&pats).build() {
                return Matcher::Packed { s, lits: lits.to_owned() };
            }
        }
        let ac = AhoCorasick::builder()
            .match_kind(aho_corasick::MatchKind::LeftmostFirst)
            .kind(Some(aho_corasick::AhoCorasickKind::DFA))
            .build(&pats)
            .unwrap();
        Matcher::AC { ac, lits: lits.to_owned() }
    }
}

#[derive(Debug)]
pub enum LiteralIter<'a> {
    Empty,
    Bytes(&'a [u8]),
    Single(&'a [u8]),
    AC(&'a [Literal]),
    Packed(&'a [Literal]),
}

impl<'a> Iterator for LiteralIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            LiteralIter::Empty => None,
            LiteralIter::Bytes(ref mut many) => {
                if many.is_empty() {
                    None
                } else {
                    let next = &many[0..1];
                    *many = &many[1..];
                    Some(next)
                }
            }
            LiteralIter::Single(ref mut one) => {
                if one.is_empty() {
                    None
                } else {
                    let next = &one[..];
                    *one = &[];
                    Some(next)
                }
            }
            LiteralIter::AC(ref mut lits) => {
                if lits.is_empty() {
                    None
                } else {
                    let next = &lits[0];
                    *lits = &lits[1..];
                    Some(next.as_bytes())
                }
            }
            LiteralIter::Packed(ref mut lits) => {
                if lits.is_empty() {
                    None
                } else {
                    let next = &lits[0];
                    *lits = &lits[1..];
                    Some(next.as_bytes())
                }
            }
        }
    }
}

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

    fn prefixes(lits: &Seq) -> SingleByteSet {
        let mut sset = SingleByteSet::new();
        let lits = match lits.literals() {
            None => return sset,
            Some(lits) => lits,
        };
        for lit in lits.iter() {
            sset.complete = sset.complete && lit.len() == 1;
            if let Some(&b) = lit.as_bytes().get(0) {
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

    fn suffixes(lits: &Seq) -> SingleByteSet {
        let mut sset = SingleByteSet::new();
        let lits = match lits.literals() {
            None => return sset,
            Some(lits) => lits,
        };
        for lit in lits.iter() {
            sset.complete = sset.complete && lit.len() == 1;
            if let Some(&b) = lit.as_bytes().last() {
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

    /// Faster find that special cases certain sizes to use memchr.
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn find(&self, text: &[u8]) -> Option<usize> {
        match self.dense.len() {
            0 => None,
            1 => memchr(self.dense[0], text),
            2 => memchr2(self.dense[0], self.dense[1], text),
            3 => memchr3(self.dense[0], self.dense[1], self.dense[2], text),
            _ => self._find(text),
        }
    }

    /// Generic find that works on any sized set.
    fn _find(&self, haystack: &[u8]) -> Option<usize> {
        for (i, &b) in haystack.iter().enumerate() {
            if self.sparse[b as usize] {
                return Some(i);
            }
        }
        None
    }

    fn approximate_size(&self) -> usize {
        (self.dense.len() * mem::size_of::<u8>())
            + (self.sparse.len() * mem::size_of::<bool>())
    }
}

/// A simple wrapper around the memchr crate's memmem implementation.
///
/// The API this exposes mirrors the API of previous substring searchers that
/// this supplanted.
#[derive(Clone, Debug)]
pub struct Memmem {
    finder: memmem::Finder<'static>,
    char_len: usize,
}

impl Memmem {
    fn new(pat: &[u8]) -> Memmem {
        Memmem {
            finder: memmem::Finder::new(pat).into_owned(),
            char_len: char_len_lossy(pat),
        }
    }

    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn find(&self, haystack: &[u8]) -> Option<usize> {
        self.finder.find(haystack)
    }

    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn is_suffix(&self, text: &[u8]) -> bool {
        if text.len() < self.len() {
            return false;
        }
        &text[text.len() - self.len()..] == self.finder.needle()
    }

    pub fn len(&self) -> usize {
        self.finder.needle().len()
    }

    pub fn char_len(&self) -> usize {
        self.char_len
    }

    fn approximate_size(&self) -> usize {
        self.finder.needle().len() * mem::size_of::<u8>()
    }
}

fn char_len_lossy(bytes: &[u8]) -> usize {
    String::from_utf8_lossy(bytes).chars().count()
}

#[cfg(test)]
mod tests_llm_16_425 {
    use super::*;

use crate::*;
    use literal::imp::LiteralSearcher;

    #[test]
    fn test_empty() {
        let searcher = LiteralSearcher::empty();
        assert!(searcher.is_empty());
        assert_eq!(searcher.len(), 0);
        assert!(!searcher.complete());
    }
}

#[cfg(test)]
mod tests_llm_16_434 {
    use super::*;

use crate::*;
    use literal::imp::{LiteralSearcher, Matcher, Memmem, SingleByteSet};
    use std::vec::Vec;

    #[test]
    fn test_literal_searcher_new() {
        // Setup
        let seq = Seq::new(vec![b"a".to_vec()]); // Assuming Seq::new() can accept Vec<u8>
        let matcher = Matcher::prefixes(&seq);
        
        // Act
        let searcher = LiteralSearcher::new(seq.clone(), matcher);
        
        // Assert
        assert!(searcher.complete(), "Searcher should be complete");
        assert_eq!(searcher.len(), 1, "Searcher length should be 1");
        assert_eq!(searcher.lcp().len(), 1, "LCP length should be 1");
        assert_eq!(searcher.lcs().len(), 1, "LCS length should be 1");
    }
}

#[cfg(test)]
mod tests_llm_16_440 {
    use super::*;

use crate::*;
    use memmem::Finder;

    #[test]
    fn test_approximate_size() {
        let pattern: &[u8] = b"test";
        let memmem = Memmem::new(pattern);
        let expected_size = pattern.len() * std::mem::size_of::<u8>();
        assert_eq!(memmem.approximate_size(), expected_size);
    }
}

#[cfg(test)]
mod tests_llm_16_441 {
    use super::*;

use crate::*;
    use literal::imp::Memmem;

    #[test]
    fn test_char_len() {
        let pattern = b"example";
        let memmem = Memmem::new(pattern);

        assert_eq!(memmem.char_len(), 7);
    }
}

#[cfg(test)]
mod tests_llm_16_442 {
    use super::*;

use crate::*;
    use literal::imp::Memmem;

    #[test]
    fn test_find_found() {
        let pattern = b"test";
        let haystack = b"This is a test string containing test.";
        let memmem = Memmem::new(pattern);
        let result = memmem.find(haystack);
        assert_eq!(result, Some(10)); // "test" starts at index 10
    }

    #[test]
    fn test_find_not_found() {
        let pattern = b"rust";
        let haystack = b"This is a test string containing test.";
        let memmem = Memmem::new(pattern);
        let result = memmem.find(haystack);
        assert_eq!(result, None); // "rust" is not in the haystack
    }

    #[test]
    fn test_find_multiple_occurrences() {
        let pattern = b"test";
        let haystack = b"This test is a test string.";
        let memmem = Memmem::new(pattern);
        let result = memmem.find(haystack);
        assert_eq!(result, Some(5)); // "test" first starts at index 5
    }

    #[test]
    fn test_find_empty_haystack() {
        let pattern = b"test";
        let haystack = b"";
        let memmem = Memmem::new(pattern);
        let result = memmem.find(haystack);
        assert_eq!(result, None); // haystack is empty, should not find anything
    }

    #[test]
    fn test_find_empty_pattern() {
        let pattern = b"";
        let haystack = b"This is a test string.";
        let memmem = Memmem::new(pattern);
        let result = memmem.find(haystack);
        assert_eq!(result, Some(0)); // empty pattern should be found at index 0
    }

    #[test]
    fn test_find_pattern_longer_than_haystack() {
        let pattern = b"longpattern";
        let haystack = b"short";
        let memmem = Memmem::new(pattern);
        let result = memmem.find(haystack);
        assert_eq!(result, None); // pattern is longer than haystack
    }
}

#[cfg(test)]
mod tests_llm_16_443 {
    use super::*;

use crate::*;
    use literal::imp::Memmem;

    #[test]
    fn test_is_suffix() {
        let pattern = b"test";
        let memmem = Memmem::new(pattern);

        assert!(memmem.is_suffix(b"this is a test"));
        assert!(memmem.is_suffix(b"test"));
        assert!(!memmem.is_suffix(b"this is a te"));
        assert!(!memmem.is_suffix(b""));
        assert!(!memmem.is_suffix(b"test test"));
    }

    #[test]
    fn test_is_suffix_with_empty_pattern() {
        let pattern = b"";
        let memmem = Memmem::new(pattern);

        assert!(memmem.is_suffix(b""));
        assert!(memmem.is_suffix(b"any string"));
    }

    #[test]
    fn test_is_suffix_with_longer_text() {
        let pattern = b"abc";
        let memmem = Memmem::new(pattern);

        assert!(memmem.is_suffix(b"xyzabc"));
        assert!(!memmem.is_suffix(b"xyzab"));
    }
}

#[cfg(test)]
mod tests_llm_16_444 {
    use super::*;

use crate::*;
    use memmem::Finder;

    #[test]
    fn test_memmem_len() {
        let pattern: &[u8] = b"test";
        let memmem = Memmem::new(pattern);
        assert_eq!(memmem.len(), pattern.len());
    }

    #[test]
    fn test_memmem_len_empty() {
        let pattern: &[u8] = b"";
        let memmem = Memmem::new(pattern);
        assert_eq!(memmem.len(), pattern.len());
    }

    #[test]
    fn test_memmem_len_unicode() {
        let pattern: &[u8] = "こんにちは".as_bytes(); // "Hello" in Japanese
        let memmem = Memmem::new(pattern);
        assert_eq!(memmem.len(), pattern.len());
    }
}

#[cfg(test)]
mod tests_llm_16_445 {
    use super::*;

use crate::*;
    use literal::imp::Memmem;

    #[test]
    fn test_memmem_new() {
        let pattern: &[u8] = b"test";
        let memmem = Memmem::new(pattern);

        assert_eq!(memmem.len(), pattern.len());
        assert_eq!(memmem.char_len(), pattern.len()); // Assuming char_len_lossy returns the length of the pattern
    }

    #[test]
    fn test_memmem_new_empty() {
        let pattern: &[u8] = b"";
        let memmem = Memmem::new(pattern);

        assert_eq!(memmem.len(), pattern.len());
        assert_eq!(memmem.char_len(), pattern.len()); // Assuming char_len_lossy returns the length of the pattern
    }
}

#[cfg(test)]
mod tests_llm_16_446 {
    use super::*;

use crate::*;
    use literal::imp::SingleByteSet;

    #[test]
    fn test_find_with_matching_byte() {
        let mut sset = SingleByteSet::new();
        sset.sparse[b'a' as usize] = true;
        let haystack = b"hello, world!";
        let result = sset._find(haystack);
        assert_eq!(result, Some(7)); // 'a' is at index 7
    }

    #[test]
    fn test_find_with_non_matching_byte() {
        let mut sset = SingleByteSet::new();
        sset.sparse[b'x' as usize] = true;
        let haystack = b"hello, world!";
        let result = sset._find(haystack);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_with_multiple_bytes() {
        let mut sset = SingleByteSet::new();
        sset.sparse[b'o' as usize] = true;
        sset.sparse[b'l' as usize] = true;
        let haystack = b"hello, world!";
        let result = sset._find(haystack);
        assert_eq!(result, Some(4)); // 'l' is at index 4
    }

    #[test]
    fn test_find_with_empty_haystack() {
        let sset = SingleByteSet::new();
        let haystack: &[u8] = b"";
        let result = sset._find(haystack);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_with_complete_set() {
        let mut sset = SingleByteSet::new();
        for byte in 0u8..=255 {
            sset.sparse[byte as usize] = true;
        }
        let haystack = b"hello";
        let result = sset._find(haystack);
        assert_eq!(result, Some(0)); // Matches first byte 'h'
    }
}

#[cfg(test)]
mod tests_llm_16_447 {
    use super::*;

use crate::*;
    use literal::imp::SingleByteSet;

    #[test]
    fn test_approximate_size_empty() {
        let sset = SingleByteSet::new();
        assert_eq!(sset.approximate_size(), 0);
    }

    #[test]
    fn test_approximate_size_dense_only() {
        let mut sset = SingleByteSet::new();
        sset.dense.push(1);
        assert_eq!(sset.approximate_size(), 1 * std::mem::size_of::<u8>());
    }

    #[test]
    fn test_approximate_size_sparse_only() {
        let mut sset = SingleByteSet::new();
        sset.sparse[1] = true;
        assert_eq!(sset.approximate_size(), 256 * std::mem::size_of::<bool>());
    }

    #[test]
    fn test_approximate_size_dense_and_sparse() {
        let mut sset = SingleByteSet::new();
        sset.dense.push(1);
        sset.sparse[1] = true;
        assert_eq!(sset.approximate_size(), std::mem::size_of::<u8>() + 256 * std::mem::size_of::<bool>());
    }

    #[test]
    fn test_approximate_size_full() {
        let mut sset = SingleByteSet::new();
        sset.dense.extend_from_slice(&[1, 2, 3]);
        for i in 0..256 {
            sset.sparse[i] = true;
        }
        assert_eq!(sset.approximate_size(), (3 * std::mem::size_of::<u8>()) + (256 * std::mem::size_of::<bool>()));
    }
}

#[cfg(test)]
mod tests_llm_16_448 {
    use super::*;

use crate::*;
    use memchr::{memchr, memchr2, memchr3};

    #[test]
    fn test_find_empty_dense() {
        let sset = SingleByteSet::new();
        let result = sset.find(b"any text");
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_single_byte() {
        let mut sset = SingleByteSet::new();
        sset.dense.push(b'a');
        let result = sset.find(b"any text with a");
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_find_two_bytes() {
        let mut sset = SingleByteSet::new();
        sset.dense.push(b'a');
        sset.dense.push(b'b');
        let result = sset.find(b"any text with a and b");
        assert_eq!(result, Some(10)); // 'a' first
    }

    #[test]
    fn test_find_three_bytes() {
        let mut sset = SingleByteSet::new();
        sset.dense.push(b'a');
        sset.dense.push(b'b');
        sset.dense.push(b'c');
        let result = sset.find(b"any text with a b and c");
        assert_eq!(result, Some(10)); // 'a' first
    }

    #[test]
    fn test_find_with_no_match() {
        let mut sset = SingleByteSet::new();
        sset.dense.push(b'x');
        let result = sset.find(b"any text");
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_with_sparse_match() {
        let mut sset = SingleByteSet::new();
        sset.sparse[b'a' as usize] = true;
        let result = sset.find(b"any text with a");
        assert_eq!(result, Some(10)); // 'a' first
    }
}

#[cfg(test)]
mod tests_llm_16_449 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_single_byte_set_new() {
        let sset = SingleByteSet::new();

        assert_eq!(sset.sparse.len(), 256);
        assert!(sset.sparse.iter().all(|&b| !b));
        assert_eq!(sset.dense.len(), 0);
        assert!(sset.complete);
        assert!(sset.all_ascii);
    }
}

#[cfg(test)]
mod tests_llm_16_452 {
    use crate::literal::imp::char_len_lossy;

    #[test]
    fn test_char_len_lossy() {
        assert_eq!(char_len_lossy(b"Hello, world!"), 13);
        assert_eq!(char_len_lossy(b"\xFF\xFF\xFF"), 0);
        assert_eq!(char_len_lossy(b"Rust \xE2\x9C\x94"), 5); // "Rust ✔"
        assert_eq!(char_len_lossy(b""), 0);
        assert_eq!(char_len_lossy(b"\xE2\x9C\x94"), 1); // "✔"
        assert_eq!(char_len_lossy(b"Valid utf8: \xC2\xA9"), 15); // "Valid utf8: ©"
    }
}
