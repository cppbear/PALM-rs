// Answer 0

#[test]
fn test_new_options_with_default_regex_options() {
    let opts = RegexOptions {
        pats: vec![],
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
    
    let exec_builder = ExecBuilder::new_options(opts.clone());
    assert_eq!(exec_builder.options, opts);
    assert!(exec_builder.match_type.is_none());
    assert!(!exec_builder.bytes);
    assert!(exec_builder.only_utf8);
}

#[test]
fn test_new_options_with_custom_regex_options() {
    let opts = RegexOptions {
        pats: vec!["test".to_string()],
        size_limit: 1024,
        dfa_size_limit: 2048,
        nest_limit: 32,
        case_insensitive: true,
        multi_line: true,
        dot_matches_new_line: true,
        swap_greed: true,
        ignore_whitespace: true,
        unicode: true,
        octal: true,
    };
    
    let exec_builder = ExecBuilder::new_options(opts.clone());
    assert_eq!(exec_builder.options, opts);
    assert!(exec_builder.match_type.is_none());
    assert!(!exec_builder.bytes);
    assert!(exec_builder.only_utf8);
}

#[test]
fn test_new_options_with_empty_patterns() {
    let opts = RegexOptions {
        pats: vec![],
        size_limit: 10,
        dfa_size_limit: 20,
        nest_limit: 5,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };

    let exec_builder = ExecBuilder::new_options(opts.clone());
    assert_eq!(exec_builder.options, opts);
    assert!(exec_builder.match_type.is_none());
    assert!(!exec_builder.bytes);
    assert!(exec_builder.only_utf8);
}

