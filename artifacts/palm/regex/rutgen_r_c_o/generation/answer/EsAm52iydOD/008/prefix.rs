// Answer 0

#[test]
fn test_build_with_one_pattern() {
    let options = RegexOptions {
        pats: vec!["a".to_string()],
        size_limit: 1024,
        dfa_size_limit: 512,
        nest_limit: 10,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: true,
        octal: false,
    };
    let builder = ExecBuilder::new_options(options);
    let _ = builder.build();
}

#[test]
fn test_build_with_multiple_patterns() {
    let options = RegexOptions {
        pats: vec!["abc".to_string(), "def".to_string(), "ghi".to_string()],
        size_limit: 2048,
        dfa_size_limit: 1024,
        nest_limit: 20,
        case_insensitive: true,
        multi_line: true,
        dot_matches_new_line: true,
        swap_greed: true,
        ignore_whitespace: true,
        unicode: true,
        octal: false,
    };
    let builder = ExecBuilder::new_options(options);
    let _ = builder.build();
}

#[test]
fn test_build_with_maximum_size_limit() {
    let options = RegexOptions {
        pats: vec!["xyz".to_string(); 10],
        size_limit: 10485760,
        dfa_size_limit: 10485760,
        nest_limit: 100,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };
    let builder = ExecBuilder::new_options(options);
    let _ = builder.build();
}

#[test]
fn test_build_with_multiple_patterns_edge_case() {
    let options = RegexOptions {
        pats: vec!["123".to_string(), "456".to_string(), "789".to_string(), "0".to_string()],
        size_limit: 4096,
        dfa_size_limit: 2048,
        nest_limit: 15,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: true,
        octal: true,
    };
    let builder = ExecBuilder::new_options(options);
    let _ = builder.build();
}

#[test]
#[should_panic]
fn test_build_with_fail_on_compilation() {
    let options = RegexOptions {
        pats: vec!["a|b".to_string(), "c|d".to_string()],
        size_limit: 1024,
        dfa_size_limit: 512,
        nest_limit: 5,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: true,
        octal: false,
    };
    let builder = ExecBuilder::new_options(options);
    let _ = builder.bounded_backtracking().build();
}

