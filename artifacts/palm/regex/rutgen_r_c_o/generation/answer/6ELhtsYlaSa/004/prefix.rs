// Answer 0

#[test]
fn test_run_cmd_suffixes() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: true,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from(""),
        arg_patterns: vec![String::from("a"), String::from("b"), String::from("c")],
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
    run(&args).unwrap();
} 

#[test]
fn test_run_cmd_suffixes_edge_case() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: true,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("a"),
        arg_patterns: vec![String::from("a")],
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
    run(&args).unwrap();
} 

#[test]
fn test_run_cmd_suffixes_empty_patterns() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: true,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from(""),
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
    run(&args).unwrap();
}

