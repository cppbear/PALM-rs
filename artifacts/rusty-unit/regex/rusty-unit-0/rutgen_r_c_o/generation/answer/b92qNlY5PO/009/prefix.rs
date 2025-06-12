// Answer 0

#[test]
fn test_cmd_literals_prefixes_all_literals_searcher() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: true,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: "a*".to_string(),
        arg_patterns: vec!["ab*", "a{2,3}"],
        arg_class: "".to_string(),
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
    cmd_literals(&args).unwrap();
}

#[test]
fn test_cmd_literals_suffixes_unambiguous_lits() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: true,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: ".*c$".to_string(),
        arg_patterns: vec!["abc", "dc"],
        arg_class: "".to_string(),
        flag_size_limit: 100,
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
    cmd_literals(&args).unwrap();
}

#[test]
fn test_cmd_literals_prefixes_all_literals_no_searcher() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: true,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: "x{1,3}".to_string(),
        arg_patterns: vec!["xyz", "xz", "xy"],
        arg_class: "".to_string(),
        flag_size_limit: 5000,
        flag_bytes: true,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: true,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: false,
    };
    cmd_literals(&args).unwrap();
}

