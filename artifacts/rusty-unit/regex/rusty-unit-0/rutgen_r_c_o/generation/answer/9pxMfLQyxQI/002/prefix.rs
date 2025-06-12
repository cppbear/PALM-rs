// Answer 0

#[test]
fn test_from_ast_case_insensitive_flag() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_multi_line_flag() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_dot_matches_new_line_flag() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_swap_greed_flag() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_unicode_flag() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_combined_flags() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
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
fn test_from_ast_no_flags() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Negation,
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_ignore_whitespace() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

