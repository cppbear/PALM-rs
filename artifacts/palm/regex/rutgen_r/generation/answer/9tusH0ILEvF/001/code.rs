// Answer 0

#[test]
fn test_check_match_no_guard_match() {
    struct Checker {
        guard: u8,
        guard_reverse_idx: usize,
        pattern: Vec<u8>,
    }

    impl Checker {
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

    let checker = Checker {
        guard: b'x', // Assume the guard we are looking for
        guard_reverse_idx: 1,
        pattern: vec![b'a', b'b', b'c'],
    };

    let haystack = b"abcdefg"; // The guard check will fail because 'g' (at index 6) != 'x'
    let window_end = 6;

    assert_eq!(checker.check_match(haystack, window_end), false);
}

#[test]
fn test_check_match_with_guard_mismatch() {
    struct Checker {
        guard: u8,
        guard_reverse_idx: usize,
        pattern: Vec<u8>,
    }

    impl Checker {
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

    let checker = Checker {
        guard: b'x', 
        guard_reverse_idx: 1,
        pattern: vec![b'a', b'b', b'c'],
    };

    let haystack = b"xyabcde"; // The guard check will pass (y == x), but the pattern will not match
    let window_end = 6;

    assert_eq!(checker.check_match(haystack, window_end), false);
}

