// Answer 0

fn test_from_ast_case_insensitive() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
        ],
    };
    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, Some(true));
    assert_eq!(flags.multi_line, Some(true));
}

fn test_from_ast_case_insensitive_negation() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
        ],
    };
    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, Some(false));
    assert_eq!(flags.multi_line, Some(true));
}

fn test_from_ast_multiple_flags_with_negation() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
        ],
    };
    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.unicode, Some(false));
    assert_eq!(flags.swap_greed, Some(false));
    assert_eq!(flags.case_insensitive, Some(false));
}

fn test_from_ast_with_only_negation() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
            },
        ],
    };
    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
} 

fn test_from_ast_empty_items() {
    let ast_flags = ast::Flags {
        items: vec![],
    };
    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

