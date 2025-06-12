// Answer 0

#[test]
fn test_check_match_success() {
    struct BoyerMooreSearchTest {
        pattern: Vec<u8>,
        skip_table: Vec<usize>,
        guard: u8,
        guard_reverse_idx: usize,
        md2_shift: usize,
    }

    impl BoyerMooreSearchTest {
        fn new(pattern: Vec<u8>) -> Self {
            let g = pattern[pattern.len() - 1];
            let gi = pattern.len() - 1;
            let skip_table = vec![0; 256]; // Simplified for this test
            let md2_shift = 1; // Simplified for this test
            BoyerMooreSearchTest {
                pattern,
                skip_table,
                guard: g,
                guard_reverse_idx: gi,
                md2_shift,
            }
        }

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
    }

    let search = BoyerMooreSearchTest::new(b"test".to_vec());
    let haystack = b"This is a test string";
    assert!(search.check_match(haystack, 15)); // "test" starts at index 10
}

#[test]
fn test_check_match_failure_guard() {
    struct BoyerMooreSearchTest {
        pattern: Vec<u8>,
        skip_table: Vec<usize>,
        guard: u8,
        guard_reverse_idx: usize,
        md2_shift: usize,
    }

    impl BoyerMooreSearchTest {
        fn new(pattern: Vec<u8>) -> Self {
            let g = pattern[pattern.len() - 1];
            let gi = pattern.len() - 1;
            let skip_table = vec![0; 256]; // Simplified for this test
            let md2_shift = 1; // Simplified for this test
            BoyerMooreSearchTest {
                pattern,
                skip_table,
                guard: g,
                guard_reverse_idx: gi,
                md2_shift,
            }
        }

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
    }

    let search = BoyerMooreSearchTest::new(b"test".to_vec());
    let haystack = b"This is a test string";
    assert!(!search.check_match(haystack, 14)); // Guard fails
}

#[test]
fn test_check_match_failure_content() {
    struct BoyerMooreSearchTest {
        pattern: Vec<u8>,
        skip_table: Vec<usize>,
        guard: u8,
        guard_reverse_idx: usize,
        md2_shift: usize,
    }

    impl BoyerMooreSearchTest {
        fn new(pattern: Vec<u8>) -> Self {
            let g = pattern[pattern.len() - 1];
            let gi = pattern.len() - 1;
            let skip_table = vec![0; 256]; // Simplified for this test
            let md2_shift = 1; // Simplified for this test
            BoyerMooreSearchTest {
                pattern,
                skip_table,
                guard: g,
                guard_reverse_idx: gi,
                md2_shift,
            }
        }

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
    }

    let search = BoyerMooreSearchTest::new(b"test".to_vec());
    let haystack = b"This is a test string";
    assert!(!search.check_match(haystack, 12)); // Content fails, should be "test"
}

