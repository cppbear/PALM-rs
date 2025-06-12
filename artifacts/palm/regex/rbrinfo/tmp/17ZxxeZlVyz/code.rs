fn find(&self, haystack: &[u8]) -> Option<usize> {
        if haystack.len() < self.pattern.len() {
            return None;
        }

        let mut window_end = self.pattern.len() - 1;

        // Inspired by the grep source. It is a way
        // to do correct loop unrolling without having to place
        // a crashpad of terminating charicters at the end in
        // the way described in the Fast String Searching paper.
        const NUM_UNROLL: usize = 10;
        // 1 for the initial position, and 1 for the md2 shift
        let short_circut = (NUM_UNROLL + 2) * self.pattern.len();

        if haystack.len() > short_circut {
            // just 1 for the md2 shift
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
                    window_end +=
                        if skip == 0 { self.md2_shift } else { skip };
                    continue;
                }
            }
        }

        // now process the input after the backstop
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