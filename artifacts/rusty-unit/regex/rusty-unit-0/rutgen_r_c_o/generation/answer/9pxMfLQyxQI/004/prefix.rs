// Answer 0

#[test]
fn test_from_ast_case_insensitive() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                span: Span::default(),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_multi_line() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
                span: Span::default(),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_dot_matches_new_line() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
                span: Span::default(),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_swap_greed() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
                span: Span::default(),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_unicode() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
                span: Span::default(),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_multiple_flags() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
                span: Span::default(),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_negation() {
    let ast = ast::Flags {
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
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_no_items() {
    let ast = ast::Flags {
        items: vec![],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_ignore_whitespace() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
                span: Span::default(),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

