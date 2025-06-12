// Answer 0

#[test]
fn test_run_cmd_ast() {
    let args = Args {
        cmd_ast: true,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("a*b"),
        arg_patterns: vec![],
        arg_class: String::from(""),
        flag_size_limit: 10485760,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: false,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: false,
    };
    
    let result = run(&args);
    assert!(result.is_ok());
}

#[test]
fn test_run_cmd_ast_invalid_pattern() {
    let args = Args {
        cmd_ast: true,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("[a-b"),
        arg_patterns: vec![],
        arg_class: String::from(""),
        flag_size_limit: 10485760,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: false,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: false,
    };
    
    let result = run(&args);
    assert!(result.is_err());
}

#[test]
fn test_run_cmd_ast_empty_pattern() {
    let args = Args {
        cmd_ast: true,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from(""), // empty pattern
        arg_patterns: vec![],
        arg_class: String::from(""),
        flag_size_limit: 10485760,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: false,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: false,
    };
    
    let result = run(&args);
    assert!(result.is_err());
}

