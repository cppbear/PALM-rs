// Answer 0

#[test]
fn test_skip_loop_with_progress() {
    struct TestStruct {
        skip_table: Vec<usize>,
        guard: u8,
        guard_reverse_idx: usize,
    }

    impl TestStruct {
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
                                window_end = window_end
                                    .checked_sub(1 + self.guard_reverse_idx)
                                    .unwrap_or(0);
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

    let test_struct = TestStruct {
        skip_table: vec![1, 0, 2, 3, 0, 1],
        guard: b'G',
        guard_reverse_idx: 2,
    };
    
    let haystack = b"ABCDEFGH";
    let backstop = 8;
    let window_end = 0;

    assert_eq!(test_struct.skip_loop(haystack, window_end, backstop), Some(8));
}

#[test]
fn test_skip_loop_with_no_progress() {
    struct TestStruct {
        skip_table: Vec<usize>,
        guard: u8,
        guard_reverse_idx: usize,
    }

    impl TestStruct {
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
                                window_end = window_end
                                    .checked_sub(1 + self.guard_reverse_idx)
                                    .unwrap_or(0);
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

    let test_struct = TestStruct {
        skip_table: vec![0, 0, 0, 0, 0, 0],
        guard: b'G',
        guard_reverse_idx: 0,
    };
    
    let haystack = b"ABCDE";
    let backstop = 10;
    let window_end = 2;

    assert_eq!(test_struct.skip_loop(haystack, window_end, backstop), Some(5));
}

