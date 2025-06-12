// Answer 0

#[test]
fn test_from_ast_case_insensitive() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }

    struct MockAst {
        items: Vec<MockFlagsItem>,
    }

    let ast = MockAst {
        items: vec![
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
            MockFlagsItem { kind: ast::FlagsItemKind::Negation },
        ],
    };
    
    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, Some(false));
    assert_eq!(result.multi_line, Some(false));
}

#[test]
fn test_from_ast_unicode_flag() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }

    struct MockAst {
        items: Vec<MockFlagsItem>,
    }

    let ast = MockAst {
        items: vec![
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode) },
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) },
            MockFlagsItem { kind: ast::FlagsItemKind::Negation },
        ],
    };

    let result = from_ast(&ast);
    assert_eq!(result.unicode, Some(false));
    assert_eq!(result.swap_greed, Some(false));
}

#[test]
fn test_from_ast_multiple_flags() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }

    struct MockAst {
        items: Vec<MockFlagsItem>,
    }

    let ast = MockAst {
        items: vec![
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
            MockFlagsItem { kind: ast::FlagsItemKind::Negation },
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) },
        ],
    };

    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, Some(false));
    assert_eq!(result.multi_line, Some(false));
    assert_eq!(result.dot_matches_new_line, Some(false));
}

#[test]
fn test_from_ast_no_items() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }

    struct MockAst {
        items: Vec<MockFlagsItem>,
    }

    let ast = MockAst { items: vec![] };

    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, None);
    assert_eq!(result.multi_line, None);
    assert_eq!(result.dot_matches_new_line, None);
    assert_eq!(result.swap_greed, None);
    assert_eq!(result.unicode, None);
}

