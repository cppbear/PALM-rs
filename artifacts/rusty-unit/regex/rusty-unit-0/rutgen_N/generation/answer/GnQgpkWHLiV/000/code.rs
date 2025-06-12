// Answer 0

#[derive(Debug)]
struct BoyerMooreSearch {
    pattern: Vec<u8>,
    skip_table: Vec<usize>,
    guard: usize,
    guard_reverse_idx: usize,
    md2_shift: Vec<usize>,
}

impl BoyerMooreSearch {
    fn select_guard(pattern: &[u8]) -> (usize, usize) {
        // Sample implementation for the test
        (pattern.len() / 2, pattern.len() - 1)
    }

    fn compile_skip_table(pattern: &[u8]) -> Vec<usize> {
        // Sample implementation for the test
        pattern.iter().map(|&b| b as usize).collect()
    }

    fn compile_md2_shift(pattern: &[u8]) -> Vec<usize> {
        // Sample implementation for the test
        pattern.iter().map(|&b| b as usize).collect()
    }

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
}

#[test]
fn test_new_non_empty_pattern() {
    let pattern = vec![b'a', b'b', b'c'];
    let searcher = BoyerMooreSearch::new(pattern.clone());
    
    assert_eq!(searcher.pattern, pattern);
    assert!(searcher.skip_table.len() > 0);
    assert!(searcher.guard >= 0);
    assert!(searcher.guard_reverse_idx >= 0);
    assert!(searcher.md2_shift.len() > 0);
}

#[should_panic]
#[test]
fn test_new_empty_pattern() {
    let pattern = vec![];
    BoyerMooreSearch::new(pattern);
}

#[test]
fn test_new_single_character_pattern() {
    let pattern = vec![b'x'];
    let searcher = BoyerMooreSearch::new(pattern.clone());
    
    assert_eq!(searcher.pattern, pattern);
    assert_eq!(searcher.skip_table.len(), 1);
    assert!(searcher.guard >= 0);
    assert!(searcher.guard_reverse_idx >= 0);
    assert_eq!(searcher.md2_shift.len(), 1);
}

