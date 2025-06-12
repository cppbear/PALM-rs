// Answer 0

fn test_from_ast_case_insensitive() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAstFlags {
        items: Vec<MockFlagsItem>,
    }

    let ast_flags = MockAstFlags {
        items: vec![
            MockFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, Some(true));
}

fn test_from_ast_multi_line() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAstFlags {
        items: Vec<MockFlagsItem>,
    }

    let ast_flags = MockAstFlags {
        items: vec![
            MockFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.multi_line, Some(true));
}

fn test_from_ast_dot_matches_new_line() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAstFlags {
        items: Vec<MockFlagsItem>,
    }

    let ast_flags = MockAstFlags {
        items: vec![
            MockFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.dot_matches_new_line, Some(true));
}

fn test_from_ast_swap_greed() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAstFlags {
        items: Vec<MockFlagsItem>,
    }

    let ast_flags = MockAstFlags {
        items: vec![
            MockFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.swap_greed, Some(true));
}

fn test_from_ast_unicode() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAstFlags {
        items: Vec<MockFlagsItem>,
    }

    let ast_flags = MockAstFlags {
        items: vec![
            MockFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.unicode, Some(true));
}

fn test_from_ast_negation() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAstFlags {
        items: Vec<MockFlagsItem>,
    }

    let ast_flags = MockAstFlags {
        items: vec![
            MockFlagsItem {
                kind: ast::FlagsItemKind::Negation,
            },
            MockFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, Some(false));
}

fn test_from_ast_ignore_whitespace() {
    struct MockFlagsItem {
        kind: ast::FlagsItemKind,
    }
    
    struct MockAstFlags {
        items: Vec<MockFlagsItem>,
    }

    let ast_flags = MockAstFlags {
        items: vec![
            MockFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
}

