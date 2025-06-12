fn skip_loop(&self,
        haystack: &[u8],
        mut window_end: usize,
        backstop: usize,
    ) -> Option<usize> {
        use std::mem;

        let window_end_snapshot = window_end;
        let skip_of = |we: usize| -> usize {
            // Unsafe might make this faster, but the benchmarks
            // were hard to interpret.
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

                        // If ten iterations did not make at least 16 words
                        // worth of progress, we just fall back on memchr.
                        if window_end - window_end_snapshot >
                             16 * mem::size_of::<usize>() {

                            // Returning a window_end >= backstop will immediatly
                            // break us out of the inner loop in `find`.
                            if window_end >= backstop {
                                return Some(window_end);
                            }

                            continue; // we made enough progress
                        } else {
                            // In case we are already there, and so that
                            // we will catch the guard char.
                            window_end = window_end
                                .checked_sub(1 + self.guard_reverse_idx)
                                .unwrap_or(0);

                            match memchr(self.guard, &haystack[window_end..]) {
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