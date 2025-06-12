// Answer 0

#[test]
fn test_check_match_success() {
    struct Matcher {
        pattern: Vec<u8>,
        guard: u8,
        guard_reverse_idx: usize,
    }

    impl Matcher {
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

    let matcher = Matcher {
        pattern: b"hello".to_vec(),
        guard: b'h',
        guard_reverse_idx: 5,
    };
    
    let haystack = b"hello world";
    let result = matcher.check_match(haystack, 5);
    assert!(result);
}

#[test]
fn test_check_match_failure_due_to_guard() {
    struct Matcher {
        pattern: Vec<u8>,
        guard: u8,
        guard_reverse_idx: usize,
    }

    impl Matcher {
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

    let matcher = Matcher {
        pattern: b"hello".to_vec(),
        guard: b'h',
        guard_reverse_idx: 5,
    };

    let haystack = b"jello world";
    let result = matcher.check_match(haystack, 5);
    assert!(!result);
}

#[test]
fn test_check_match_failure_due_to_pattern_mismatch() {
    struct Matcher {
        pattern: Vec<u8>,
        guard: u8,
        guard_reverse_idx: usize,
    }

    impl Matcher {
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
    
    let matcher = Matcher {
        pattern: b"hello".to_vec(),
        guard: b'h',
        guard_reverse_idx: 5,
    };

    let haystack = b"hella world";
    let result = matcher.check_match(haystack, 5);
    assert!(!result);
}

#[test]
fn test_check_match_boundary_condition() {
    struct Matcher {
        pattern: Vec<u8>,
        guard: u8,
        guard_reverse_idx: usize,
    }

    impl Matcher {
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

    let matcher = Matcher {
        pattern: b"hello".to_vec(),
        guard: b'h',
        guard_reverse_idx: 5,
    };

    let haystack = b"line with hello";
    let result = matcher.check_match(haystack, 14); // ensured to match "hello"
    assert!(result);
}

