fn compile_md2_shift(pattern: &[u8]) -> usize {
        let shiftc = *pattern.last().unwrap();

        // For a pattern of length 1 we will never apply the
        // shift rule, so we use a poison value on the principle
        // that failing fast is a good thing.
        if pattern.len() == 1 {
            return 0xDEADBEAF;
        }

        let mut i = pattern.len() - 2;
        while i > 0 {
            if pattern[i] == shiftc {
                return (pattern.len() - 1) - i;
            }
            i -= 1;
        }

        // The skip char never re-occurs in the pattern, so
        // we can just shift the whole window length.
        pattern.len() - 1
    }