// Answer 0

#[test]
fn test_from_ast_case_insensitive_enabled() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_case_insensitive_disabled() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Negation,
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_multi_line_enabled() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_swap_greed_enabled() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_no_flags() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Negation,
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_ignore_whitespace_enabled() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_empty_ast() {
    let ast = ast::Flags {
        items: vec![],
    };
    let flags = Flags::from_ast(&ast);
}

