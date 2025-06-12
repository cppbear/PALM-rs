// Answer 0

fn test_from_ast_case_insensitive() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { 
                span: Span::default(), 
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) 
            },
            ast::FlagsItem { 
                span: Span::default(), 
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) 
            },
        ],
    };

    let result = Flags::from_ast(&ast_flags);
    assert_eq!(result.case_insensitive, Some(true));
    assert_eq!(result.multi_line, Some(true));
}

fn test_from_ast_multi_line_with_negation() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { 
                span: Span::default(), 
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) 
            },
            ast::FlagsItem { 
                span: Span::default(), 
                kind: ast::FlagsItemKind::Negation 
            },
            ast::FlagsItem { 
                span: Span::default(), 
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) 
            },
        ],
    };

    let result = Flags::from_ast(&ast_flags);
    assert_eq!(result.case_insensitive, Some(true));
    assert_eq!(result.multi_line, Some(false));
}

fn test_from_ast_empty() {
    let ast_flags = ast::Flags {
        items: vec![],
    };

    let result = Flags::from_ast(&ast_flags);
    assert_eq!(result.case_insensitive, None);
    assert_eq!(result.multi_line, None);
    assert_eq!(result.dot_matches_new_line, None);
    assert_eq!(result.swap_greed, None);
    assert_eq!(result.unicode, None);
}

fn test_from_ast_flags_with_unicode() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { 
                span: Span::default(), 
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode) 
            },
            ast::FlagsItem { 
                span: Span::default(), 
                kind: ast::FlagsItemKind::Negation 
            },
        ],
    };

    let result = Flags::from_ast(&ast_flags);
    assert_eq!(result.unicode, Some(false));
}

fn test_from_ast_multiple_flags() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { 
                span: Span::default(), 
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode) 
            },
            ast::FlagsItem { 
                span: Span::default(), 
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) 
            },
            ast::FlagsItem { 
                span: Span::default(), 
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) 
            },
            ast::FlagsItem { 
                span: Span::default(), 
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) 
            },
        ],
    };

    let result = Flags::from_ast(&ast_flags);
    assert_eq!(result.case_insensitive, Some(true));
    assert_eq!(result.unicode, Some(true));
    assert_eq!(result.dot_matches_new_line, Some(true));
    assert_eq!(result.swap_greed, Some(true));
}

