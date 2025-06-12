// Answer 0

#[test]
fn test_check_match_with_non_matching_pattern() {
    struct TestBoyerMooreSearch {
        pattern: Vec<u8>,
        guard: u8,
        guard_reverse_idx: usize,
    }
    
    impl BoyerMooreSearch {
        fn new_test(pattern: Vec<u8>) -> Self {
            let guard = pattern[0];
            let guard_reverse_idx = 0;
            let skip_table = vec![0; pattern.len()];
            let md2_shift = 0;
            BoyerMooreSearch {
                pattern,
                skip_table,
                guard,
                guard_reverse_idx,
                md2_shift,
            }
        }
    }

    let pattern = b"abc".to_vec();    
    let haystack = b"abcdxyz";
    let window_end = 3;

    let bms = BoyerMooreSearch::new_test(pattern);
    
    // The guard condition is satisfied since the last character checked is 'c'
    // which matches the guard, and this pattern does not match the haystack.
    assert_eq!(bms.check_match(haystack, window_end), false);
}

#[test]
fn test_check_match_with_matching_guard_but_non_matching_pattern() {
    struct TestBoyerMooreSearch {
        pattern: Vec<u8>,
        guard: u8,
        guard_reverse_idx: usize,
    }
    
    impl BoyerMooreSearch {
        fn new_test(pattern: Vec<u8>) -> Self {
            let guard = pattern[0];
            let guard_reverse_idx = 0;
            let skip_table = vec![0; pattern.len()];
            let md2_shift = 0;
            BoyerMooreSearch {
                pattern,
                skip_table,
                guard,
                guard_reverse_idx,
                md2_shift,
            }
        }
    }

    let pattern = b"xyz".to_vec();    
    let haystack = b"abcdefgzyx";
    let window_end = 10; // To check against the 'z'

    let bms = BoyerMooreSearch::new_test(pattern);

    // In this case, the guard is 'x' but the characters don't match the pattern.
    assert_eq!(bms.check_match(haystack, window_end), false);
}

