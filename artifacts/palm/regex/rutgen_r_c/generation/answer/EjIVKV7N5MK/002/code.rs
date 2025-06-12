// Answer 0

#[test]
fn test_literals_empty_exprs() {
    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from(".*"),
        arg_patterns: Vec::new(),
        arg_class: String::from("[a-z]"),
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

    let exprs: Vec<Hir> = Vec::new();
    let get_literals = |_: &mut Literals, _: &Hir| false;

    let result = args.literals(&exprs, get_literals);

    // Since there are no expressions, it should return the empty literals initialized with the limits.
    let expected_literals = args.empty_literals();
    assert_eq!(result.limit_size(), expected_literals.limit_size());
    assert_eq!(result.limit_class(), expected_literals.limit_class());
}

#[test]
fn test_literals_single_expr_with_false() {
    struct DummyHir;

    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from(".*"),
        arg_patterns: Vec::new(),
        arg_class: String::from("[a-z]"),
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

    let exprs = vec![DummyHir];
    let get_literals = |_: &mut Literals, _: &DummyHir| false;

    let result = args.literals(&exprs, get_literals);

    // It should return empty literals as the callback always returns false.
    let expected_literals = args.empty_literals();
    assert_eq!(result.limit_size(), expected_literals.limit_size());
    assert_eq!(result.limit_class(), expected_literals.limit_class());
}

#[test]
fn test_literals_multiple_exprs_with_failure() {
    struct DummyHir;

    let args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from(".*"),
        arg_patterns: Vec::new(),
        arg_class: String::from("[a-z]"),
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

    let exprs = vec![DummyHir, DummyHir, DummyHir];
    let get_literals = |_: &mut Literals, _: &DummyHir| false; // Always return false

    let result = args.literals(&exprs, get_literals);

    // It should still return empty literals due to the get_literals function returning false for all.
    let expected_literals = args.empty_literals();
    assert_eq!(result.limit_size(), expected_literals.limit_size());
    assert_eq!(result.limit_class(), expected_literals.limit_class());
}

