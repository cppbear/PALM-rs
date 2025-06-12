// Answer 0

#[test]
fn test_from_ast_case_insensitive() {
    struct FlagsItem {
        kind: ast::FlagsItemKind,
    }
    struct Ast {
        items: Vec<FlagsItem>,
    }
    let ast = Ast {
        items: vec![
            FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
        ],
    };
    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, Some(true));
}

#[test]
fn test_from_ast_multi_line() {
    struct FlagsItem {
        kind: ast::FlagsItemKind,
    }
    struct Ast {
        items: Vec<FlagsItem>,
    }
    let ast = Ast {
        items: vec![
            FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
        ],
    };
    let result = from_ast(&ast);
    assert_eq!(result.multi_line, Some(true));
}

#[test]
fn test_from_ast_dot_matches_new_line() {
    struct FlagsItem {
        kind: ast::FlagsItemKind,
    }
    struct Ast {
        items: Vec<FlagsItem>,
    }
    let ast = Ast {
        items: vec![
            FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) },
        ],
    };
    let result = from_ast(&ast);
    assert_eq!(result.dot_matches_new_line, Some(true));
}

#[test]
fn test_from_ast_swap_greed() {
    struct FlagsItem {
        kind: ast::FlagsItemKind,
    }
    struct Ast {
        items: Vec<FlagsItem>,
    }
    let ast = Ast {
        items: vec![
            FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) },
        ],
    };
    let result = from_ast(&ast);
    assert_eq!(result.swap_greed, Some(true));
}

#[test]
fn test_from_ast_unicode() {
    struct FlagsItem {
        kind: ast::FlagsItemKind,
    }
    struct Ast {
        items: Vec<FlagsItem>,
    }
    let ast = Ast {
        items: vec![
            FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode) },
        ],
    };
    let result = from_ast(&ast);
    assert_eq!(result.unicode, Some(true));
}

#[test]
fn test_from_ast_negation() {
    struct FlagsItem {
        kind: ast::FlagsItemKind,
    }
    struct Ast {
        items: Vec<FlagsItem>,
    }
    let ast = Ast {
        items: vec![
            FlagsItem { kind: ast::FlagsItemKind::Negation },
            FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
        ],
    };
    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, Some(false));
}

#[test]
fn test_from_ast_ignore_whitespace() {
    struct FlagsItem {
        kind: ast::FlagsItemKind,
    }
    struct Ast {
        items: Vec<FlagsItem>,
    }
    let ast = Ast {
        items: vec![
            FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace) },
        ],
    };
    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, None);
}

