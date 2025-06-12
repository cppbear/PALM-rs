// Answer 0

#[test]
fn test_new_options_empty_patterns() {
    let opts = RegexOptions {
        pats: vec![String::from("")],
        size_limit: 0,
        dfa_size_limit: 0,
        nest_limit: 0,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };
    let builder = ExecBuilder::new_options(opts);
}

#[test]
fn test_new_options_non_empty_patterns() {
    let opts = RegexOptions {
        pats: vec![String::from("abc"), String::from("123")],
        size_limit: 256,
        dfa_size_limit: 128,
        nest_limit: 5,
        case_insensitive: true,
        multi_line: true,
        dot_matches_new_line: true,
        swap_greed: true,
        ignore_whitespace: true,
        unicode: true,
        octal: true,
    };
    let builder = ExecBuilder::new_options(opts);
}

#[test]
fn test_new_options_max_limits() {
    let opts = RegexOptions {
        pats: vec![String::from(".*")],
        size_limit: usize::MAX,
        dfa_size_limit: usize::MAX,
        nest_limit: u32::MAX,
        case_insensitive: true,
        multi_line: true,
        dot_matches_new_line: true,
        swap_greed: true,
        ignore_whitespace: true,
        unicode: true,
        octal: true,
    };
    let builder = ExecBuilder::new_options(opts);
}

#[test]
fn test_new_options_zero_limits() {
    let opts = RegexOptions {
        pats: vec![String::from("xyz")],
        size_limit: 0,
        dfa_size_limit: 0,
        nest_limit: 0,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };
    let builder = ExecBuilder::new_options(opts);
}

#[test]
fn test_new_options_single_pattern() {
    let opts = RegexOptions {
        pats: vec![String::from("a")],
        size_limit: 10,
        dfa_size_limit: 10,
        nest_limit: 1,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };
    let builder = ExecBuilder::new_options(opts);
}

