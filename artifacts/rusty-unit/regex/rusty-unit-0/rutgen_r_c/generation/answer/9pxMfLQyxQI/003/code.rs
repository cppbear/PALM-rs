// Answer 0

fn test_from_ast_case_insensitive() {
    struct Span;
    struct TestFlags {
        items: Vec<FlagsItem>,
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
    
    let ast = TestFlags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::DotMatchesNewLine) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::SwapGreed) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::Unicode) },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(true));
    assert_eq!(flags.multi_line, Some(true));
    assert_eq!(flags.dot_matches_new_line, Some(true));
    assert_eq!(flags.swap_greed, Some(true));
    assert_eq!(flags.unicode, Some(true));
}

fn test_from_ast_negation() {
    struct Span;
    struct TestFlags {
        items: Vec<FlagsItem>,
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

    let ast = TestFlags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::Negation) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(false));
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

fn test_from_ast_ignore_whitespace() {
    struct Span;
    struct TestFlags {
        items: Vec<FlagsItem>,
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
    
    let ast = TestFlags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::SwapGreed) },
        ],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.swap_greed, Some(true));
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.unicode, None);
}

fn test_from_ast_empty() {
    struct Span;
    struct TestFlags {
        items: Vec<FlagsItem>,
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
    
    let ast = TestFlags {
        items: vec![],
    };

    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

