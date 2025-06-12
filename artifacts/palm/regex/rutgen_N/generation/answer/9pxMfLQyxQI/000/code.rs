// Answer 0

#[test]
fn test_from_ast_case_insensitive_enabled() {
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

        pub struct FlagsItem {
            pub kind: FlagsItemKind,
        }

        pub enum FlagsItemKind {
            Negation,
            Flag(Flag),
        }

        #[derive(Clone, Copy)]
        pub enum Flag {
            CaseInsensitive,
            MultiLine,
            DotMatchesNewLine,
            SwapGreed,
            Unicode,
            IgnoreWhitespace,
        }
    }

    let input = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
        ],
    };

    let result = from_ast(&input);
    assert_eq!(result.case_insensitive, Some(true));
}

#[test]
fn test_from_ast_multi_line_disabled() {
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

        pub struct FlagsItem {
            pub kind: FlagsItemKind,
        }

        pub enum FlagsItemKind {
            Negation,
            Flag(Flag),
        }

        #[derive(Clone, Copy)]
        pub enum Flag {
            CaseInsensitive,
            MultiLine,
            DotMatchesNewLine,
            SwapGreed,
            Unicode,
            IgnoreWhitespace,
        }
    }

    let input = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
        ],
    };

    let result = from_ast(&input);
    assert_eq!(result.multi_line, Some(false));
}

#[test]
fn test_from_ast_combined_flags() {
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

        pub struct FlagsItem {
            pub kind: FlagsItemKind,
        }

        pub enum FlagsItemKind {
            Negation,
            Flag(Flag),
        }

        #[derive(Clone, Copy)]
        pub enum Flag {
            CaseInsensitive,
            MultiLine,
            DotMatchesNewLine,
            SwapGreed,
            Unicode,
            IgnoreWhitespace,
        }
    }

    let input = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
            },
        ],
    };

    let result = from_ast(&input);
    assert_eq!(result.case_insensitive, Some(true));
    assert_eq!(result.multi_line, Some(true));
    assert_eq!(result.dot_matches_new_line, Some(true));
}

