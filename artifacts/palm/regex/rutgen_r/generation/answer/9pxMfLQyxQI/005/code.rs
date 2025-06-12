// Answer 0

fn test_from_ast_case_insensitive() {
    struct Flags {
        case_insensitive: Option<bool>,
        multi_line: Option<bool>,
        dot_matches_new_line: Option<bool>,
        swap_greed: Option<bool>,
        unicode: Option<bool>,
    }

    struct FlagsItem {
        kind: FlagsItemKind,
    }

    enum FlagsItemKind {
        Negation,
        Flag(Flag),
    }

    enum Flag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }

    struct Ast {
        items: Vec<FlagsItem>,
    }

    fn from_ast(ast: &Ast) -> Flags {
        let mut flags = Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        };
        let mut enable = true;
        for item in &ast.items {
            match item.kind {
                FlagsItemKind::Negation => {
                    enable = false;
                }
                FlagsItemKind::Flag(Flag::CaseInsensitive) => {
                    flags.case_insensitive = Some(enable);
                }
                FlagsItemKind::Flag(Flag::MultiLine) => {
                    flags.multi_line = Some(enable);
                }
                FlagsItemKind::Flag(Flag::DotMatchesNewLine) => {
                    flags.dot_matches_new_line = Some(enable);
                }
                FlagsItemKind::Flag(Flag::SwapGreed) => {
                    flags.swap_greed = Some(enable);
                }
                FlagsItemKind::Flag(Flag::Unicode) => {
                    flags.unicode = Some(enable);
                }
                FlagsItemKind::Flag(Flag::IgnoreWhitespace) => {}
            }
        }
        flags
    }

    let ast = Ast {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
        ],
    };

    let flags = from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(true));
    assert_eq!(flags.multi_line, Some(true));
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

fn test_from_ast_multiple_flags() {
    struct Flags {
        case_insensitive: Option<bool>,
        multi_line: Option<bool>,
        dot_matches_new_line: Option<bool>,
        swap_greed: Option<bool>,
        unicode: Option<bool>,
    }

    struct FlagsItem {
        kind: FlagsItemKind,
    }

    enum FlagsItemKind {
        Negation,
        Flag(Flag),
    }

    enum Flag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }

    struct Ast {
        items: Vec<FlagsItem>,
    }

    fn from_ast(ast: &Ast) -> Flags {
        let mut flags = Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        };
        let mut enable = true;
        for item in &ast.items {
            match item.kind {
                FlagsItemKind::Negation => {
                    enable = false;
                }
                FlagsItemKind::Flag(Flag::CaseInsensitive) => {
                    flags.case_insensitive = Some(enable);
                }
                FlagsItemKind::Flag(Flag::MultiLine) => {
                    flags.multi_line = Some(enable);
                }
                FlagsItemKind::Flag(Flag::DotMatchesNewLine) => {
                    flags.dot_matches_new_line = Some(enable);
                }
                FlagsItemKind::Flag(Flag::SwapGreed) => {
                    flags.swap_greed = Some(enable);
                }
                FlagsItemKind::Flag(Flag::Unicode) => {
                    flags.unicode = Some(enable);
                }
                FlagsItemKind::Flag(Flag::IgnoreWhitespace) => {}
            }
        }
        flags
    }

    let ast = Ast {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::Unicode) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::SwapGreed) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
        ],
    };

    let flags = from_ast(&ast);
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, Some(true));
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, Some(true));
    assert_eq!(flags.unicode, Some(true));
}

fn test_from_ast_negation() {
    struct Flags {
        case_insensitive: Option<bool>,
        multi_line: Option<bool>,
        dot_matches_new_line: Option<bool>,
        swap_greed: Option<bool>,
        unicode: Option<bool>,
    }

    struct FlagsItem {
        kind: FlagsItemKind,
    }

    enum FlagsItemKind {
        Negation,
        Flag(Flag),
    }

    enum Flag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }

    struct Ast {
        items: Vec<FlagsItem>,
    }

    fn from_ast(ast: &Ast) -> Flags {
        let mut flags = Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        };
        let mut enable = true;
        for item in &ast.items {
            match item.kind {
                FlagsItemKind::Negation => {
                    enable = false;
                }
                FlagsItemKind::Flag(Flag::CaseInsensitive) => {
                    flags.case_insensitive = Some(enable);
                }
                FlagsItemKind::Flag(Flag::MultiLine) => {
                    flags.multi_line = Some(enable);
                }
                FlagsItemKind::Flag(Flag::DotMatchesNewLine) => {
                    flags.dot_matches_new_line = Some(enable);
                }
                FlagsItemKind::Flag(Flag::SwapGreed) => {
                    flags.swap_greed = Some(enable);
                }
                FlagsItemKind::Flag(Flag::Unicode) => {
                    flags.unicode = Some(enable);
                }
                FlagsItemKind::Flag(Flag::IgnoreWhitespace) => {}
            }
        }
        flags
    }

    let ast = Ast {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Negation },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
        ],
    };

    let flags = from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(false));
    assert_eq!(flags.multi_line, Some(false));
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

