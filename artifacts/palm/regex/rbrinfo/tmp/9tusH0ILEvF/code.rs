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