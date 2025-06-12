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
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(true));
}

#[test]
fn test_from_ast_multi_line() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAst {
        items: Vec<MockFlagsItem>,
    }

    let ast = MockAst {
        items: vec![
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.multi_line, Some(true));
}

#[test]
fn test_from_ast_dot_matches_new_line() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAst {
        items: Vec<MockFlagsItem>,
    }

    let ast = MockAst {
        items: vec![
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.dot_matches_new_line, Some(true));
}

#[test]
fn test_from_ast_swap_greed() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAst {
        items: Vec<MockFlagsItem>,
    }

    let ast = MockAst {
        items: vec![
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.swap_greed, Some(true));
}

#[test]
fn test_from_ast_unicode() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAst {
        items: Vec<MockFlagsItem>,
    }

    let ast = MockAst {
        items: vec![
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode) },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.unicode, Some(true));
}

#[test]
fn test_from_ast_negation() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAst {
        items: Vec<MockFlagsItem>,
    }

    let ast = MockAst {
        items: vec![
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
            MockFlagsItem { kind: ast::FlagsItemKind::Negation },
            MockFlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(true));
    assert_eq!(flags.multi_line, Some(false));
}

