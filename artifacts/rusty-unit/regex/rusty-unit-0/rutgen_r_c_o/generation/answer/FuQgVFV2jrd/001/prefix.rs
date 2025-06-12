// Answer 0

#[test]
fn test_parse_one_empty_pattern() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
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
    args.parse_one();
}

#[test]
fn test_parse_one_single_character() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("a"),
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
    args.parse_one();
}

#[test]
fn test_parse_one_long_pattern() {
    let long_pattern = "a".repeat(10000);
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: long_pattern,
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
    args.parse_one();
}

#[test]
fn test_parse_one_special_characters() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$"),
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
    args.parse_one();
}

#[test]
fn test_parse_one_exceeding_limits() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: true,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("a"),
        arg_patterns: vec![],
        arg_class: String::from(""),
        flag_size_limit: 1,
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
    args.parse_one();
}

