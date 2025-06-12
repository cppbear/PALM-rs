fn select_guard(pattern: &[u8]) -> (u8, usize) {
        let mut rarest = pattern[0];
        let mut rarest_rev_idx = pattern.len() - 1;
        for (i, c) in pattern.iter().enumerate() {
            if freq_rank(*c) < freq_rank(rarest) {
                rarest = *c;
                rarest_rev_idx = (pattern.len() - 1) - i;
            }
        }

        (rarest, rarest_rev_idx)
    }