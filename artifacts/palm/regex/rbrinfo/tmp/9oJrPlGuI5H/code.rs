fn should_use(pattern: &[u8]) -> bool {
        // The minimum pattern length required to use TBM.
        const MIN_LEN: usize = 9;
        // The minimum frequency rank (lower is rarer) that every byte in the
        // pattern must have in order to use TBM. That is, if the pattern
        // contains _any_ byte with a lower rank, then TBM won't be used.
        const MIN_CUTOFF: usize = 150;
        // The maximum frequency rank for any byte.
        const MAX_CUTOFF: usize = 255;
        // The scaling factor used to determine the actual cutoff frequency
        // to use (keeping in mind that the minimum frequency rank is bounded
        // by MIN_CUTOFF). This scaling factor is an attempt to make TBM more
        // likely to be used as the pattern grows longer. That is, longer
        // patterns permit somewhat less frequent bytes than shorter patterns,
        // under the assumption that TBM gets better as the pattern gets
        // longer.
        const LEN_CUTOFF_PROPORTION: usize = 4;

        let scaled_rank = pattern.len().wrapping_mul(LEN_CUTOFF_PROPORTION);
        let cutoff = cmp::max(
            MIN_CUTOFF,
            MAX_CUTOFF - cmp::min(MAX_CUTOFF, scaled_rank),
        );
        // The pattern must be long enough to be worthwhile. e.g., memchr will
        // be faster on `e` because it is short even though e is quite common.
        pattern.len() > MIN_LEN
            // all the bytes must be more common than the cutoff.
            && pattern.iter().all(|c| freq_rank(*c) >= cutoff)
    }