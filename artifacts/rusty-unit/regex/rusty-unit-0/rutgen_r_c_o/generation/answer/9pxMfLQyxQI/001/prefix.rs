// Answer 0

#[test]
fn test_all_flags_enabled() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive), span: Span::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine), span: Span::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine), span: Span::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed), span: Span::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode), span: Span::default() }
        ]
    };
    Flags::from_ast(&ast_flags);
}

#[test]
fn test_with_negation_flag() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation, span: Span::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive), span: Span::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine), span: Span::default() }
        ]
    };
    Flags::from_ast(&ast_flags);
}

#[test]
fn test_only_ignore_whitespace() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace), span: Span::default() }
        ]
    };
    Flags::from_ast(&ast_flags);
}

#[test]
fn test_mixed_flags_with_negation() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive), span: Span::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation, span: Span::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode), span: Span::default() }
        ]
    };
    Flags::from_ast(&ast_flags);
}

#[test]
fn test_no_flags() {
    let ast_flags = ast::Flags {
        items: vec![]
    };
    Flags::from_ast(&ast_flags);
}

