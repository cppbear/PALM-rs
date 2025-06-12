// Answer 0

#[test]
fn test_from_ast_no_items() {
    let ast = ast::Flags {
        items: vec![],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_single_negation() {
    let ast = ast::Flags {
        items: vec![ast::FlagsItem {
            kind: ast::FlagsItemKind::Negation,
            span: Span::default(),
        }],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_single_case_insensitive() {
    let ast = ast::Flags {
        items: vec![ast::FlagsItem {
            kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            span: Span::default(),
        }],
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
        ],
    };
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_with_negation() {
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
fn test_from_ast_combined_flags() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
                span: Span::default(),
            },
        ],
    };
    let flags = Flags::from_ast(&ast);
}

