// Answer 0

#[test]
fn test_check_match_false_due_to_guard() {
    struct PatternMatcher {
        guard: u8,
        guard_reverse_idx: usize,
        pattern: Vec<u8>,
    }

    impl PatternMatcher {
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
        guard: b'A',
        guard_reverse_idx: 1,
        pattern: vec![b'B', b'C'],
    };
    
    let haystack = vec![b'A', b'X', b'Y', b'Z'];
    let window_end = 3; // Position of 'Z'

    assert_eq!(matcher.check_match(&haystack, window_end), false);
}

#[test]
fn test_check_match_false_due_to_pattern_mismatch() {
    struct PatternMatcher {
        guard: u8,
        guard_reverse_idx: usize,
        pattern: Vec<u8>,
    }

    impl PatternMatcher {
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
        guard: b'A',
        guard_reverse_idx: 1,
        pattern: vec![b'B', b'C'],
    };
    
    let haystack = vec![b'A', b'B', b'D', b'Z'];
    let window_end = 3; // Position of 'D'

    assert_eq!(matcher.check_match(&haystack, window_end), false);
}

