// Answer 0

#[test]
fn test_check_match_valid_case() {
    struct PatternMatcher<'a> {
        pattern: &'a [u8],
        guard: u8,
        guard_reverse_idx: usize,
    }

    impl<'a> PatternMatcher<'a> {
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

    let matcher = PatternMatcher {
        pattern: b"abc",
        guard: b'x',
        guard_reverse_idx: 1,
    };

    let haystack = b"xxabc";
    let window_end = 5; // 5 is the index of the last 'x' in "xxabc"

    // Check that the match is successful
    assert!(matcher.check_match(haystack, window_end));
}

#[test]
fn test_check_match_guard_failure() {
    struct PatternMatcher<'a> {
        pattern: &'a [u8],
        guard: u8,
        guard_reverse_idx: usize,
    }

    impl<'a> PatternMatcher<'a> {
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

    let matcher = PatternMatcher {
        pattern: b"abc",
        guard: b'x',
        guard_reverse_idx: 1,
    };

    let haystack = b"xxabx"; // last character fails guard check
    let window_end = 5;

    // Check that the match fails due to incorrect guard
    assert!(!matcher.check_match(haystack, window_end));
}

#[test]
fn test_check_match_empty_pattern() {
    struct PatternMatcher<'a> {
        pattern: &'a [u8],
        guard: u8,
        guard_reverse_idx: usize,
    }

    impl<'a> PatternMatcher<'a> {
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

    let matcher = PatternMatcher {
        pattern: b"",
        guard: b'x',
        guard_reverse_idx: 1,
    };

    let haystack = b"xx"; 
    let window_end = 2; 

    // Expecting true as any position should match an empty pattern
    assert!(matcher.check_match(haystack, window_end));
}

