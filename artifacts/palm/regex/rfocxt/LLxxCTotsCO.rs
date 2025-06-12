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
    fn check_match(&self, haystack: &[u8], window_end: usize) -> bool {
        if haystack[window_end - self.guard_reverse_idx] != self.guard {
            return false;
        }
        let window_start = window_end - (self.pattern.len() - 1);
        for i in 0..self.pattern.len() {
            if self.pattern[i] != haystack[window_start + i] {
                return false;
            }
        }
        true
    }
    #[inline]
    fn skip_loop(
        &self,
        haystack: &[u8],
        mut window_end: usize,
        backstop: usize,
    ) -> Option<usize> {
        use std::mem;
        let window_end_snapshot = window_end;
        let skip_of = |we: usize| -> usize { self.skip_table[haystack[we] as usize] };
        loop {
            let mut skip = skip_of(window_end);
            window_end += skip;
            skip = skip_of(window_end);
            window_end += skip;
            if skip != 0 {
                skip = skip_of(window_end);
                window_end += skip;
                skip = skip_of(window_end);
                window_end += skip;
                skip = skip_of(window_end);
                window_end += skip;
                if skip != 0 {
                    skip = skip_of(window_end);
                    window_end += skip;
                    skip = skip_of(window_end);
                    window_end += skip;
                    skip = skip_of(window_end);
                    window_end += skip;
                    if skip != 0 {
                        skip = skip_of(window_end);
                        window_end += skip;
                        skip = skip_of(window_end);
                        window_end += skip;
                        if window_end - window_end_snapshot
                            > 16 * mem::size_of::<usize>()
                        {
                            if window_end >= backstop {
                                return Some(window_end);
                            }
                            continue;
                        } else {
                            window_end = window_end
                                .checked_sub(1 + self.guard_reverse_idx)
                                .unwrap_or(0);
                            match memchr(self.guard, &haystack[window_end..]) {
                                None => return None,
                                Some(g_idx) => {
                                    return Some(window_end + g_idx + self.guard_reverse_idx);
                                }
                            }
                        }
                    }
                }
            }
            return Some(window_end);
        }
    }
    fn compile_skip_table(pattern: &[u8]) -> Vec<usize> {}
    fn select_guard(pattern: &[u8]) -> (u8, usize) {}
    fn compile_md2_shift(pattern: &[u8]) -> usize {}
    fn approximate_size(&self) -> usize {}
}
