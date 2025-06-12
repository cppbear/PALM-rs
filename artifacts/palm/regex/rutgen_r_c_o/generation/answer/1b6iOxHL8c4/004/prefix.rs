// Answer 0

#[test]
fn test_boyer_moore_find_basic_match() {
    let pattern = b"abc".to_vec();
    let haystack = b"xyzabcxyz".to_vec();
    let skip_table = vec![0; 256]; // Simplified skip table, actual would be generated
    let guard = pattern[pattern.len() - 1];
    let guard_reverse_idx = pattern.len() - 1;
    let md2_shift = 1; // Simplified value, actual would be based on the pattern
    let matcher = BoyerMooreSearch {
        pattern,
        skip_table,
        guard,
        guard_reverse_idx,
        md2_shift,
    };
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::BoyerMoore(matcher),
    };
    
    searcher.find(&haystack);
}

#[test]
fn test_boyer_moore_find_no_match() {
    let pattern = b"def".to_vec();
    let haystack = b"xyzabcxyz".to_vec();
    let skip_table = vec![0; 256];
    let guard = pattern[pattern.len() - 1];
    let guard_reverse_idx = pattern.len() - 1;
    let md2_shift = 1;
    let matcher = BoyerMooreSearch {
        pattern,
        skip_table,
        guard,
        guard_reverse_idx,
        md2_shift,
    };
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::BoyerMoore(matcher),
    };
    
    searcher.find(&haystack);
}

#[test]
fn test_boyer_moore_find_empty_haystack() {
    let pattern = b"abc".to_vec();
    let haystack = b"".to_vec();
    let skip_table = vec![0; 256];
    let guard = pattern[pattern.len() - 1];
    let guard_reverse_idx = pattern.len() - 1;
    let md2_shift = 1;
    let matcher = BoyerMooreSearch {
        pattern,
        skip_table,
        guard,
        guard_reverse_idx,
        md2_shift,
    };
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::BoyerMoore(matcher),
    };
    
    searcher.find(&haystack);
}

#[test]
fn test_boyer_moore_find_exact_match() {
    let pattern = b"test".to_vec();
    let haystack = b"this is a test string".to_vec();
    let skip_table = vec![0; 256];
    let guard = pattern[pattern.len() - 1];
    let guard_reverse_idx = pattern.len() - 1;
    let md2_shift = 1;
    let matcher = BoyerMooreSearch {
        pattern,
        skip_table,
        guard,
        guard_reverse_idx,
        md2_shift,
    };
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::BoyerMoore(matcher),
    };
    
    searcher.find(&haystack);
}

#[test]
fn test_boyer_moore_find_multiple_occurrences() {
    let pattern = b"abc".to_vec();
    let haystack = b"abcxyzabc".to_vec();
    let skip_table = vec![0; 256];
    let guard = pattern[pattern.len() - 1];
    let guard_reverse_idx = pattern.len() - 1;
    let md2_shift = 1;
    let matcher = BoyerMooreSearch {
        pattern,
        skip_table,
        guard,
        guard_reverse_idx,
        md2_shift,
    };
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::BoyerMoore(matcher),
    };
    
    searcher.find(&haystack);
}

#[test]
fn test_boyer_moore_find_large_haystack() {
    let pattern = b"pattern".to_vec();
    let haystack = vec![b'a'; 1_048_576]; // Large haystack
    let skip_table = vec![0; 256];
    let guard = pattern[pattern.len() - 1];
    let guard_reverse_idx = pattern.len() - 1;
    let md2_shift = 1;
    let matcher = BoyerMooreSearch {
        pattern,
        skip_table,
        guard,
        guard_reverse_idx,
        md2_shift,
    };
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::BoyerMoore(matcher),
    };
    
    searcher.find(&haystack);
}

