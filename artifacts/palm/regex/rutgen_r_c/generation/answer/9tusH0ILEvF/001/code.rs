// Answer 0

#[test]
fn test_check_match_guard_condition_false() {
    struct TestBoyerMooreSearch {
        guard: u8,
        guard_reverse_idx: usize,
        pattern: Vec<u8>,
    }

    impl TestBoyerMooreSearch {
        fn check_match(&self, haystack: &[u8], window_end: usize) -> bool {
            // guard test
            if haystack[window_end - self.guard_reverse_idx] != self.guard {
                return false;
            }

            // match loop
            let window_start = window_end - (self.pattern.len() - 1);
            for i in 0..self.pattern.len() {
                if self.pattern[i] != haystack[window_start + i] {
                    return false;
                }
            }

            true
        }
    }

    let pattern = b"abc".to_vec();
    let guard = b'z'; // selecting a guard that is not in the haystack
    let search = TestBoyerMooreSearch {
        guard,
        guard_reverse_idx: 1,
        pattern: pattern.clone(),
    };

    let haystack = b"xyzabc";
    let window_end = haystack.len(); // Position at the end of haystack
    assert_eq!(search.check_match(haystack, window_end), false);
}

