// Answer 0

#[derive(Debug)]
struct SkipTable {
    skip_table: Vec<usize>,
    guard: u8,
    guard_reverse_idx: usize,
}

impl SkipTable {
    fn new(skip_table: Vec<usize>, guard: u8, guard_reverse_idx: usize) -> Self {
        Self {
            skip_table,
            guard,
            guard_reverse_idx,
        }
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

                        if window_end - window_end_snapshot >
                             16 * mem::size_of::<usize>() {
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

#[test]
fn test_skip_loop() {
    let skip_table = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let guard = b'g';
    let guard_reverse_idx = 2;
    let haystack = b"abcdeedcbaageg";
    let window_end = 12; // Start just before the guard character
    let backstop = 15; // Limit the search

    let st = SkipTable::new(skip_table, guard, guard_reverse_idx);
    let result = st.skip_loop(haystack, window_end, backstop);
    
    assert_eq!(result, Some(14)); // Expected return value
}

#[test]
fn test_skip_loop_at_bound() {
    let skip_table = vec![0; 256]; // Assuming all characters skip 0
    let guard = b'g';
    let guard_reverse_idx = 0; 
    let haystack = b"abcdeedcba";
    let window_end = 8; // Set to the end of haystack
    let backstop = 10; // Limit for backstop 

    let st = SkipTable::new(skip_table, guard, guard_reverse_idx);
    let result = st.skip_loop(haystack, window_end, backstop);
    
    assert_eq!(result, Some(10)); // Expected return at backstop point
}

#[test]
fn test_skip_loop_with_no_guard() {
    let skip_table = vec![1; 256]; // Each character has a skip of 1
    let guard = b'g';
    let guard_reverse_idx = 1; // Offset for the guard
    let haystack = b"abcdefgh";
    let window_end = 7; // Last character
    let backstop = 10; // Further than haystack

    let st = SkipTable::new(skip_table, guard, guard_reverse_idx);
    let result = st.skip_loop(haystack, window_end, backstop);
    
    assert_eq!(result, Some(8)); // Return after processing entire haystack
}

