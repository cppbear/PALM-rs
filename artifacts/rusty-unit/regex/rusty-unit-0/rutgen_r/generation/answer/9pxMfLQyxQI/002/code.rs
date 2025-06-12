// Answer 0

#[test]
fn test_from_ast_case_insensitive() {
    struct AstFlags {
        items: Vec<AstFlagsItem>,
    }

    enum AstFlagsItemKind {
        Negation,
        Flag(AstFlag),
    }

    enum AstFlag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }

    struct AstFlagsItem {
        kind: AstFlagsItemKind,
    }

    struct Flags {
        case_insensitive: Option<bool>,
        multi_line: Option<bool>,
        dot_matches_new_line: Option<bool>,
        swap_greed: Option<bool>,
        unicode: Option<bool>,
    }

    impl Default for Flags {
        fn default() -> Self {
            Flags {
                case_insensitive: None,
                multi_line: None,
                dot_matches_new_line: None,
                swap_greed: None,
                unicode: None,
            }
        }
    }

    let ast = AstFlags {
        items: vec![
            AstFlagsItem { kind: AstFlagsItemKind::Flag(AstFlag::CaseInsensitive) },
            AstFlagsItem { kind: AstFlagsItemKind::Flag(AstFlag::Unicode) },
        ],
    };

    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, Some(true));
    assert_eq!(result.unicode, Some(true));
}

#[test]
fn test_from_ast_negation() {
    struct AstFlags {
        items: Vec<AstFlagsItem>,
    }

    enum AstFlagsItemKind {
        Negation,
        Flag(AstFlag),
    }

    enum AstFlag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }

    struct AstFlagsItem {
        kind: AstFlagsItemKind,
    }

    struct Flags {
        case_insensitive: Option<bool>,
        multi_line: Option<bool>,
        dot_matches_new_line: Option<bool>,
        swap_greed: Option<bool>,
        unicode: Option<bool>,
    }

    impl Default for Flags {
        fn default() -> Self {
            Flags {
                case_insensitive: None,
                multi_line: None,
                dot_matches_new_line: None,
                swap_greed: None,
                unicode: None,
            }
        }
    }

    let ast = AstFlags {
        items: vec![
            AstFlagsItem { kind: AstFlagsItemKind::Negation },
            AstFlagsItem { kind: AstFlagsItemKind::Flag(AstFlag::CaseInsensitive) },
            AstFlagsItem { kind: AstFlagsItemKind::Flag(AstFlag::MultiLine) },
        ],
    };

    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, Some(false));
    assert_eq!(result.multi_line, Some(false));
}

#[test]
fn test_from_ast_no_items() {
    struct AstFlags {
        items: Vec<AstFlagsItem>,
    }

    struct AstFlagsItem;

    struct Flags {
        case_insensitive: Option<bool>,
        multi_line: Option<bool>,
        dot_matches_new_line: Option<bool>,
        swap_greed: Option<bool>,
        unicode: Option<bool>,
    }

    impl Default for Flags {
        fn default() -> Self {
            Flags {
                case_insensitive: None,
                multi_line: None,
                dot_matches_new_line: None,
                swap_greed: None,
                unicode: None,
            }
        }
    }

    let ast = AstFlags { items: vec![] };
    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, None);
    assert_eq!(result.multi_line, None);
    assert_eq!(result.unicode, None);
}

