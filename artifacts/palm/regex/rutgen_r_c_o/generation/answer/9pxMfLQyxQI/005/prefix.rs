// Answer 0

#[test]
fn test_flags_from_ast_case_insensitive() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            }
        ]
    };
    
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_multi_line() {
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
        ]
    };
    
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_unicode() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Negation,
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
            }
        ]
    };
    
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_dot_matches_new_line() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Negation,
            }
        ]
    };
    
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_swap_greed() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
            }
        ]
    };
    
    let flags = Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_empty() {
    let ast = ast::Flags {
        items: vec![]
    };
    
    let flags = Flags::from_ast(&ast);
} 

#[test]
fn test_flags_from_ast_ignore_whitespace() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Negation,
            }
        ]
    };
    
    let flags = Flags::from_ast(&ast);
}

