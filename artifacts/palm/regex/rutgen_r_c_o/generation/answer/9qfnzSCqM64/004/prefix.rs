// Answer 0

#[test]
fn test_run_with_valid_args_ast() {
    let args = Args {
        cmd_ast: true,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("a"),
        arg_patterns: Vec::new(),
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
    let _ = run(&args);
}

#[test]
fn test_run_with_valid_args_hir() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: true,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("b"),
        arg_patterns: Vec::new(),
        arg_class: String::from(""),
        flag_size_limit: 1024,
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
    let _ = run(&args);
}

#[test]
fn test_run_with_valid_args_prefixes() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: true,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from(""),
        arg_patterns: vec![String::from("abc"), String::from("abcd")],
        arg_class: String::from(""),
        flag_size_limit: 2048,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: true,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: true,
        flag_lcs: false,
        flag_searcher: false,
    };
    let _ = run(&args);
}

#[test]
fn test_run_with_valid_args_suffixes() {
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
        arg_patterns: vec![String::from("xyz"), String::from("xyzabc")],
        arg_class: String::from(""),
        flag_size_limit: 4096,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: true,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: true,
        flag_searcher: false,
    };
    let _ = run(&args);
}

#[test]
fn test_run_with_valid_args_anchors() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: true,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("^a$"),
        arg_patterns: Vec::new(),
        arg_class: String::from(""),
        flag_size_limit: 512,
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
    let _ = run(&args);
}

#[test]
fn test_run_with_valid_args_captures() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: true,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("(a|b)"),
        arg_patterns: Vec::new(),
        arg_class: String::from(""),
        flag_size_limit: 10000,
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
    let _ = run(&args);
}

#[test]
fn test_run_with_valid_args_compile() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: true,
        cmd_utf8_ranges: false,
        arg_pattern: String::from(""),
        arg_patterns: vec![String::from("x+y"), String::from("abc")],
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
    let _ = run(&args);
}

#[test]
fn test_run_with_valid_args_utf8_ranges() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: true,
        arg_pattern: String::from(""),
        arg_patterns: Vec::new(),
        arg_class: String::from("a-z"),
        flag_size_limit: 32768,
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
    let _ = run(&args);
}

