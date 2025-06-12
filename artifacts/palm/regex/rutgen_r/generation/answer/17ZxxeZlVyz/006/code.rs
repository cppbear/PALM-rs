// Answer 0

#[test]
fn test_find_with_exact_match_and_large_haystack() {
    struct TestStruct {
        pattern: Vec<u8>,
        skip_table: Vec<usize>,
        md2_shift: usize,
    }

    impl TestStruct {
        fn skip_loop(&self, haystack: &[u8], window_end: usize, backstop: usize) -> Option<usize> {
            for i in (0..=window_end).rev() {
                if haystack[i] == self.pattern[0] {
                    return Some(i);
                }
                if i == backstop {
                    break;
                }
            }
            None
        }

        fn check_match(&self, haystack: &[u8], window_end: usize) -> bool {
            haystack[window_end - (self.pattern.len() - 1)..=window_end] == self.pattern[..]
        }

        fn find(&self, haystack: &[u8]) -> Option<usize> {
            if haystack.len() < self.pattern.len() {
                return None;
            }

            let mut window_end = self.pattern.len() - 1;
            const NUM_UNROLL: usize = 10;
            let short_circut = (NUM_UNROLL + 2) * self.pattern.len();

            if haystack.len() > short_circut {
                let backstop = haystack.len() - ((NUM_UNROLL + 1) * self.pattern.len());
                loop {
                    window_end = match self.skip_loop(haystack, window_end, backstop) {
                        Some(i) => i,
                        None => return None,
                    };
                    if window_end >= backstop {
                        break;
                    }
                    if self.check_match(haystack, window_end) {
                        return Some(window_end - (self.pattern.len() - 1));
                    } else {
                        let skip = self.skip_table[haystack[window_end] as usize];
                        window_end += if skip == 0 { self.md2_shift } else { skip };
                        continue;
                    }
                }
            }

            while window_end < haystack.len() {
                let mut skip = self.skip_table[haystack[window_end] as usize];
                if skip == 0 {
                    if self.check_match(haystack, window_end) {
                        return Some(window_end - (self.pattern.len() - 1));
                    } else {
                        skip = self.md2_shift;
                    }
                }
                window_end += skip;
            }

            None
        }
    }

    let haystack = b"this is a long example haystack that definitely has the pattern somewhere";
    let pattern = b"pattern".to_vec();
    let skip_table = vec![0; 256]; // Assume all bytes have skip value of 0 for simplicity
    let md2_shift = 1;

    let test_struct = TestStruct {
        pattern,
        skip_table,
        md2_shift,
    };

    let result = test_struct.find(haystack);
    assert_eq!(result, Some(51));
}

