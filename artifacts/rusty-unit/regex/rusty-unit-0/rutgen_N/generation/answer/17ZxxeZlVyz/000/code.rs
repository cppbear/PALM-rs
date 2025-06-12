// Answer 0

#[test]
fn test_find_pattern_found() {
    struct PatternFinder {
        pattern: Vec<u8>,
        skip_table: Vec<usize>,
        md2_shift: usize,
    }

    impl PatternFinder {
        fn check_match(&self, haystack: &[u8], window_end: usize) -> bool {
            haystack[window_end - (self.pattern.len() - 1)..=window_end] == self.pattern[..]
        }

        fn skip_loop(&self, haystack: &[u8], window_end: usize, backstop: usize) -> Option<usize> {
            while window_end > backstop {
                if self.skip_table[haystack[window_end] as usize] == 0 {
                    return Some(window_end);
                }
                window_end -= 1;
            }
            None
        }
    }

    let pattern_finder = PatternFinder {
        pattern: b"abc".to_vec(),
        skip_table: vec![0; 256], // assuming all characters are not in the pattern
        md2_shift: 1,
    };

    let haystack = b"xyzabc";
    assert_eq!(pattern_finder.find(haystack), Some(3));
}

#[test]
fn test_find_pattern_not_found() {
    struct PatternFinder {
        pattern: Vec<u8>,
        skip_table: Vec<usize>,
        md2_shift: usize,
    }

    impl PatternFinder {
        fn check_match(&self, haystack: &[u8], window_end: usize) -> bool {
            haystack[window_end - (self.pattern.len() - 1)..=window_end] == self.pattern[..]
        }

        fn skip_loop(&self, haystack: &[u8], window_end: usize, backstop: usize) -> Option<usize> {
            while window_end > backstop {
                if self.skip_table[haystack[window_end] as usize] == 0 {
                    return Some(window_end);
                }
                window_end -= 1;
            }
            None
        }
    }

    let pattern_finder = PatternFinder {
        pattern: b"abc".to_vec(),
        skip_table: vec![0; 256],
        md2_shift: 1,
    };

    let haystack = b"xyzdef";
    assert_eq!(pattern_finder.find(haystack), None);
}

#[test]
fn test_find_pattern_too_short() {
    struct PatternFinder {
        pattern: Vec<u8>,
        skip_table: Vec<usize>,
        md2_shift: usize,
    }

    impl PatternFinder {
        fn check_match(&self, haystack: &[u8], window_end: usize) -> bool {
            haystack[window_end - (self.pattern.len() - 1)..=window_end] == self.pattern[..]
        }

        fn skip_loop(&self, haystack: &[u8], window_end: usize, backstop: usize) -> Option<usize> {
            while window_end > backstop {
                if self.skip_table[haystack[window_end] as usize] == 0 {
                    return Some(window_end);
                }
                window_end -= 1;
            }
            None
        }
    }

    let pattern_finder = PatternFinder {
        pattern: b"abc".to_vec(),
        skip_table: vec![0; 256],
        md2_shift: 1,
    };

    let haystack = b"ab";
    assert_eq!(pattern_finder.find(haystack), None);
}

