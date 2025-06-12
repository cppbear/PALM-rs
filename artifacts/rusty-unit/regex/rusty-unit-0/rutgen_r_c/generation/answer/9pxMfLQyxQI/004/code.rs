// Answer 0

fn test_from_ast_case_insensitive() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                // Assume appropriate Span is provided
                span: Span::default(),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, Some(true));
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

fn test_from_ast_multi_line() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
                span: Span::default(),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, Some(true));
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

fn test_from_ast_dot_matches_new_line() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
                span: Span::default(),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, Some(true));
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

fn test_from_ast_swap_greed() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
                span: Span::default(),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, Some(true));
    assert_eq!(flags.unicode, None);
}

fn test_from_ast_unicode() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
                span: Span::default(),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, Some(true));
}

fn test_from_ast_negation() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                span: Span::default(),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, Some(false));
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

fn test_from_ast_multiple_flags() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
                span: Span::default(),
            },
        ],
    };

    let flags = Flags::from_ast(&ast_flags);
    assert_eq!(flags.case_insensitive, Some(true));
    assert_eq!(flags.multi_line, Some(true));
}

