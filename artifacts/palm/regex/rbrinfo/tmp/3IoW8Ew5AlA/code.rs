fn default() -> Self {
        RegexOptions {
            pats: vec![],
            size_limit: 10 * (1<<20),
            dfa_size_limit: 2 * (1<<20),
            nest_limit: 250,
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            ignore_whitespace: false,
            unicode: true,
            octal: false,
        }
    }