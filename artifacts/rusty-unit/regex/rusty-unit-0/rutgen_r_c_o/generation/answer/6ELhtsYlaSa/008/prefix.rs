// Answer 0

#[test]
fn test_run_utf8_ranges_with_single_class() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: true,
        arg_pattern: String::new(),
        arg_patterns: Vec::new(),
        arg_class: String::from("a"),
        flag_size_limit: 1,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: false,
        flag_literal_limit: 1,
        flag_class_limit: 1,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: false,
    };
    run(&args).unwrap();
}

#[test]
fn test_run_utf8_ranges_with_multiple_classes() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: true,
        arg_pattern: String::new(),
        arg_patterns: Vec::new(),
        arg_class: String::from("a-z"),
        flag_size_limit: 10485760,
        flag_bytes: true,
        flag_dfa: true,
        flag_dfa_reverse: false,
        flag_all_literals: false,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: true,
        flag_lcs: false,
        flag_searcher: false,
    };
    run(&args).unwrap();
}

#[test]
fn test_run_utf8_ranges_with_edge_class_limit() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: true,
        arg_pattern: String::new(),
        arg_patterns: Vec::new(),
        arg_class: String::from("b"),
        flag_size_limit: 10485760,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: false,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: true,
        flag_searcher: false,
    };
    run(&args).unwrap();
}

#[test]
fn test_run_utf8_ranges_with_empty_class() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: true,
        arg_pattern: String::new(),
        arg_patterns: Vec::new(),
        arg_class: String::new(),
        flag_size_limit: 10485760,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: false,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: true,
    };
    let result = run(&args);
    assert!(result.is_err()); // Expected to return an error due to empty class
}

