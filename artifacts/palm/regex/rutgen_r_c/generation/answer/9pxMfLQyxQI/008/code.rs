// Answer 0

#[test]
fn test_flags_from_ast_case_insensitive() {
    struct TestItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestItem>,
    }

    let ast = TestAst {
        items: vec![
            TestItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
            TestItem { kind: ast::FlagsItemKind::Negation },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(false));
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

#[test]
fn test_flags_from_ast_multi_line() {
    struct TestItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestItem>,
    }

    let ast = TestAst {
        items: vec![
            TestItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
            TestItem { kind: ast::FlagsItemKind::Negation },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.multi_line, Some(false));
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

#[test]
fn test_flags_from_ast_dot_matches_new_line() {
    struct TestItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestItem>,
    }

    let ast = TestAst {
        items: vec![
            TestItem { kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) },
            TestItem { kind: ast::FlagsItemKind::Negation },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.dot_matches_new_line, Some(false));
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

#[test]
fn test_flags_from_ast_swap_greed() {
    struct TestItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestItem>,
    }

    let ast = TestAst {
        items: vec![
            TestItem { kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) },
            TestItem { kind: ast::FlagsItemKind::Negation },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.swap_greed, Some(false));
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.unicode, None);
}

#[test]
fn test_flags_from_ast_unicode() {
    struct TestItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestItem>,
    }

    let ast = TestAst {
        items: vec![
            TestItem { kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode) },
            TestItem { kind: ast::FlagsItemKind::Negation },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.unicode, Some(false));
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
}

#[test]
fn test_flags_from_ast_multiple_items() {
    struct TestItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestItem>,
    }

    let ast = TestAst {
        items: vec![
            TestItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
            TestItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
            TestItem { kind: ast::FlagsItemKind::Negation },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(false));
    assert_eq!(flags.multi_line, Some(false));
}

#[test]
fn test_flags_from_ast_ignore_whitespace() {
    struct TestItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestItem>,
    }

    let ast = TestAst {
        items: vec![
            TestItem { kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace) },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

