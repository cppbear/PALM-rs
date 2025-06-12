fn compile_skip_table(pattern: &[u8]) -> Vec<usize> {
        let mut tab = vec![pattern.len(); 256];

        // For every char in the pattern, we write a skip
        // that will line us up with the rightmost occurrence.
        //
        // N.B. the sentinel (0) is written by the last
        // loop iteration.
        for (i, c) in pattern.iter().enumerate() {
            tab[*c as usize] = (pattern.len() - 1) - i;
        }

        tab
    }