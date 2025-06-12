fn new(pattern: Vec<u8>) -> Self {
        debug_assert!(pattern.len() > 0);

        let (g, gi) = Self::select_guard(pattern.as_slice());
        let skip_table = Self::compile_skip_table(pattern.as_slice());
        let md2_shift = Self::compile_md2_shift(pattern.as_slice());
        BoyerMooreSearch {
            pattern: pattern,
            skip_table: skip_table,
            guard: g,
            guard_reverse_idx: gi,
            md2_shift: md2_shift,
        }
    }