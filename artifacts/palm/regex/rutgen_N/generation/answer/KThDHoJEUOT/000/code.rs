// Answer 0

#[test]
fn test_skip_loop_found() {
    struct TestStruct {
        skip_table: Vec<usize>,
        guard: u8,
        guard_reverse_idx: usize,
    }

    impl TestStruct {
        fn new(skip_table: Vec<usize>, guard: u8, guard_reverse_idx: usize) -> Self {
            Self { skip_table, guard, guard_reverse_idx }
        }

        fn skip_loop(&self, haystack: &[u8], mut window_end: usize, backstop: usize) -> Option<usize> {
            use std::mem;

            let window_end_snapshot = window_end;
            let skip_of = |we: usize| -> usize {
                self.skip_table[haystack[we] as usize]
            };

            loop {
                let mut skip = skip_of(window_end); window_end += skip;
                skip = skip_of(window_end); window_end += skip;
                if skip != 0 {
                    skip = skip_of(window_end); window_end += skip;
                    skip = skip_of(window_end); window_end += skip;
                    skip = skip_of(window_end); window_end += skip;
                    if skip != 0 {
                        skip = skip_of(window_end); window_end += skip;
                        skip = skip_of(window_end); window_end += skip;
                        skip = skip_of(window_end); window_end += skip;
                        if skip != 0 {
                            skip = skip_of(window_end); window_end += skip;
                            skip = skip_of(window_end); window_end += skip;

                            if window_end - window_end_snapshot > 16 * mem::size_of::<usize>() {
                                if window_end >= backstop {
                                    return Some(window_end);
                                }
                                continue;
                            } else {
                                window_end = window_end.checked_sub(1 + self.guard_reverse_idx).unwrap_or(0);
                                match memchr::memchr(self.guard, &haystack[window_end..]) {
                                    None => return None,
                                    Some(g_idx) => {
                                        return Some(window_end + g_idx + self.guard_reverse_idx);
                                    }
                                }
                            }
                        }
                    }
                }

                return Some(window_end);
            }
        }
    }

    let skip_table = vec![0, 1, 2, 3, 4, 5];
    let guard = b'g'; // Example guard character.
    let guard_reverse_idx = 1;
    
    let tester = TestStruct::new(skip_table, guard, guard_reverse_idx);
    let haystack = b"hello world";
    let result = tester.skip_loop(haystack, 5, 10);
    assert_eq!(result, Some(11));
}

#[test]
fn test_skip_loop_not_found() {
    struct TestStruct {
        skip_table: Vec<usize>,
        guard: u8,
        guard_reverse_idx: usize,
    }

    impl TestStruct {
        fn new(skip_table: Vec<usize>, guard: u8, guard_reverse_idx: usize) -> Self {
            Self { skip_table, guard, guard_reverse_idx }
        }

        fn skip_loop(&self, haystack: &[u8], mut window_end: usize, backstop: usize) -> Option<usize> {
            use std::mem;

            let window_end_snapshot = window_end;
            let skip_of = |we: usize| -> usize {
                self.skip_table[haystack[we] as usize]
            };

            loop {
                let mut skip = skip_of(window_end); window_end += skip;
                skip = skip_of(window_end); window_end += skip;
                if skip != 0 {
                    skip = skip_of(window_end); window_end += skip;
                    skip = skip_of(window_end); window_end += skip;
                    skip = skip_of(window_end); window_end += skip;
                    if skip != 0 {
                        skip = skip_of(window_end); window_end += skip;
                        skip = skip_of(window_end); window_end += skip;
                        skip = skip_of(window_end); window_end += skip;
                        if skip != 0 {
                            skip = skip_of(window_end); window_end += skip;
                            skip = skip_of(window_end); window_end += skip;

                            if window_end - window_end_snapshot > 16 * mem::size_of::<usize>() {
                                if window_end >= backstop {
                                    return Some(window_end);
                                }
                                continue;
                            } else {
                                window_end = window_end.checked_sub(1 + self.guard_reverse_idx).unwrap_or(0);
                                match memchr::memchr(self.guard, &haystack[window_end..]) {
                                    None => return None,
                                    Some(g_idx) => {
                                        return Some(window_end + g_idx + self.guard_reverse_idx);
                                    }
                                }
                            }
                        }
                    }
                }

                return Some(window_end);
            }
        }
    }

    let skip_table = vec![0, 1, 2, 3, 4, 5];
    let guard = b'g'; // Example guard character.
    let guard_reverse_idx = 1;

    let tester = TestStruct::new(skip_table, guard, guard_reverse_idx);
    let haystack = b"hello";
    let result = tester.skip_loop(haystack, 5, 10);
    assert_eq!(result, None);
}

