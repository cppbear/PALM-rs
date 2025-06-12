// Answer 0

#[test]
#[should_panic]
fn test_run_with_dfa_reverse_and_invalid_args() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: "a*".to_string(),
        arg_patterns: vec!["pattern1".to_string(), "pattern2".to_string()],
        arg_class: "class1".to_string(),
        flag_size_limit: 10485760,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: true,
        flag_all_literals: false,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: false,
    };
    let _ = run(&args);
}

#[test]
#[should_panic]
fn test_run_with_dfa_reverse_and_zero_size_limit() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: "a*".to_string(),
        arg_patterns: vec!["pattern1".to_string()],
        arg_class: "class1".to_string(),
        flag_size_limit: 0,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: true,
        flag_all_literals: false,
        flag_literal_limit: 100,
        flag_class_limit: 5,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: false,
    };
    let _ = run(&args);
}

#[test]
#[should_panic]
fn test_run_with_dfa_reverse_and_exceeding_literal_limit() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: "abc".to_string(),
        arg_patterns: vec!["a".to_string(); 300],
        arg_class: "class2".to_string(),
        flag_size_limit: 10485760,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: true,
        flag_all_literals: false,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: false,
    };
    let _ = run(&args);
}

