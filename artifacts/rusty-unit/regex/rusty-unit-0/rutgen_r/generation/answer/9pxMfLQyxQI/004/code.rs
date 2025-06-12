// Answer 0

#[derive(Default)]
struct Flags {
    case_insensitive: Option<bool>,
    multi_line: Option<bool>,
    dot_matches_new_line: Option<bool>,
    swap_greed: Option<bool>,
    unicode: Option<bool>,
}

mod ast {
    pub struct Flags {
        pub items: Vec<FlagsItem>,
    }

    pub enum FlagsItemKind {
        Negation,
        Flag(Flag),
    }

    pub enum Flag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }

    pub struct FlagsItem {
        pub kind: FlagsItemKind,
    }
}

fn from_ast(ast: &ast::Flags) -> Flags {
    let mut flags = Flags::default();
    let mut enable = true;
    for item in &ast.items {
        match item.kind {
            ast::FlagsItemKind::Negation => {
                enable = false;
            }
            ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) => {
                flags.case_insensitive = Some(enable);
            }
            ast::FlagsItemKind::Flag(ast::Flag::MultiLine) => {
                flags.multi_line = Some(enable);
            }
            ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) => {
                flags.dot_matches_new_line = Some(enable);
            }
            ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) => {
                flags.swap_greed = Some(enable);
            }
            ast::FlagsItemKind::Flag(ast::Flag::Unicode) => {
                flags.unicode = Some(enable);
            }
            ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace) => {}
        }
    }
    flags
}

#[test]
fn test_from_ast_with_case_insensitive() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) },
        ],
    };
    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, Some(true));
    assert_eq!(result.multi_line, Some(true));
    assert_eq!(result.dot_matches_new_line, Some(true));
}

#[test]
fn test_from_ast_with_negation() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
        ],
    };
    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, Some(false));
    assert_eq!(result.multi_line, Some(false));
}

#[test]
fn test_from_ast_with_no_items() {
    let ast = ast::Flags {
        items: Vec::new(),
    };
    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, None);
    assert_eq!(result.multi_line, None);
    assert_eq!(result.dot_matches_new_line, None);
}

