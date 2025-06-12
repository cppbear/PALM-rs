// Answer 0

#[test]
fn test_flags_from_ast_case_insensitive() {
    let ast_item = ast::FlagsItem {
        span: Span::default(),
        kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
    };
    let ast = ast::Flags {
        items: vec![ast_item],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_multi_line() {
    let ast_item = ast::FlagsItem {
        span: Span::default(),
        kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
    };
    let ast = ast::Flags {
        items: vec![ast_item],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_dot_matches_new_line() {
    let ast_item = ast::FlagsItem {
        span: Span::default(),
        kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
    };
    let ast = ast::Flags {
        items: vec![ast_item],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_swap_greed() {
    let ast_item = ast::FlagsItem {
        span: Span::default(),
        kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
    };
    let ast = ast::Flags {
        items: vec![ast_item],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_unicode() {
    let ast_item = ast::FlagsItem {
        span: Span::default(),
        kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
    };
    let ast = ast::Flags {
        items: vec![ast_item],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_multiple_flags() {
    let ast_items = vec![
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
        },
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
        },
    ];
    let ast = ast::Flags {
        items: ast_items,
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_with_negation() {
    let ast_items = vec![
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Negation,
        },
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
        },
    ];
    let ast = ast::Flags {
        items: ast_items,
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_ignore_whitespace() {
    let ast_item = ast::FlagsItem {
        span: Span::default(),
        kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
    };
    let ast = ast::Flags {
        items: vec![ast_item],
    };
    Flags::from_ast(&ast);
} 

#[test]
fn test_flags_from_ast_empty() {
    let ast = ast::Flags {
        items: vec![],
    };
    Flags::from_ast(&ast);
}

