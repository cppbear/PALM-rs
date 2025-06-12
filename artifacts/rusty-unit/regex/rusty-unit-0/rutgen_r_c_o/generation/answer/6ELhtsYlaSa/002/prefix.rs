// Answer 0

#[test]
fn test_run_cmd_hir_with_valid_pattern() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: true,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("a*b*"),
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
        flag_searcher: false,
    };
    run(&args).unwrap();
}

#[test]
fn test_run_cmd_hir_with_empty_pattern() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: true,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from(""),
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
        flag_searcher: false,
    };
    run(&args).unwrap();
}

#[test]
fn test_run_cmd_hir_with_special_characters() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: true,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from(".*?"),
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
        flag_searcher: false,
    };
    run(&args).unwrap();
}

#[test]
fn test_run_cmd_hir_with_complex_pattern() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: true,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("a(b|c)*d+"),
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
        flag_searcher: false,
    };
    run(&args).unwrap();
}

