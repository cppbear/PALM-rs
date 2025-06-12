// Answer 0

fn test_from_ast_case_insensitive() {
    #[derive(Clone, Debug)]
    struct Span;
    
    #[derive(Clone, Debug)]
    enum Flag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }
    
    #[derive(Clone, Debug)]
    enum FlagsItemKind {
        Flag(Flag),
        Negation,
    }
    
    #[derive(Clone, Debug)]
    struct FlagsItem {
        kind: FlagsItemKind,
    }
    
    #[derive(Clone, Debug)]
    struct AstFlags {
        items: Vec<FlagsItem>,
    }
    
    let ast = AstFlags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
        ],
    };
    
    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(true));
    assert_eq!(flags.multi_line, Some(true));
}

fn test_from_ast_multi_line() {
    #[derive(Clone, Debug)]
    struct Span;
    
    #[derive(Clone, Debug)]
    enum Flag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }
    
    #[derive(Clone, Debug)]
    enum FlagsItemKind {
        Flag(Flag),
        Negation,
    }
    
    #[derive(Clone, Debug)]
    struct FlagsItem {
        kind: FlagsItemKind,
    }
    
    #[derive(Clone, Debug)]
    struct AstFlags {
        items: Vec<FlagsItem>,
    }
    
    let ast = AstFlags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::DotMatchesNewLine) },
        ],
    };
    
    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.multi_line, Some(true));
    assert_eq!(flags.dot_matches_new_line, Some(true));
}

fn test_from_ast_combined_flags() {
    #[derive(Clone, Debug)]
    struct Span;
    
    #[derive(Clone, Debug)]
    enum Flag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }
    
    #[derive(Clone, Debug)]
    enum FlagsItemKind {
        Flag(Flag),
        Negation,
    }
    
    #[derive(Clone, Debug)]
    struct FlagsItem {
        kind: FlagsItemKind,
    }
    
    #[derive(Clone, Debug)]
    struct AstFlags {
        items: Vec<FlagsItem>,
    }
    
    let ast = AstFlags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::Unicode) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::Negation) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
        ],
    };
    
    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(true));
    assert_eq!(flags.multi_line, Some(false));
    assert_eq!(flags.unicode, Some(true));
}

fn test_from_ast_no_flags() {
    #[derive(Clone, Debug)]
    struct Span;
    
    #[derive(Clone, Debug)]
    enum Flag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }
    
    #[derive(Clone, Debug)]
    enum FlagsItemKind {
        Flag(Flag),
        Negation,
    }
    
    #[derive(Clone, Debug)]
    struct FlagsItem {
        kind: FlagsItemKind,
    }
    
    #[derive(Clone, Debug)]
    struct AstFlags {
        items: Vec<FlagsItem>,
    }
    
    let ast = AstFlags { items: vec![] };
    
    let flags = Flags::from_ast(&ast);
    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.unicode, None);
}

