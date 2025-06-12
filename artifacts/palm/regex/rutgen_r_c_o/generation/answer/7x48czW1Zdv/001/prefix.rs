// Answer 0

#[test]
fn test_set_flags_case_insensitive() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let ast_flags = ast::Flags {
        items: vec![ast::FlagsItem {
            kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
        }],
    };

    let translator_i = TranslatorI::new(&trans, "example_pattern");
    translator_i.set_flags(&ast_flags);
}

#[test]
fn test_set_flags_multi_line() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let ast_flags = ast::Flags {
        items: vec![ast::FlagsItem {
            kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
        }],
    };

    let translator_i = TranslatorI::new(&trans, "example_pattern");
    translator_i.set_flags(&ast_flags);
}

#[test]
fn test_set_flags_dot_matches_new_line() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let ast_flags = ast::Flags {
        items: vec![ast::FlagsItem {
            kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
        }],
    };

    let translator_i = TranslatorI::new(&trans, "example_pattern");
    translator_i.set_flags(&ast_flags);
}

#[test]
fn test_set_flags_swap_greed() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let ast_flags = ast::Flags {
        items: vec![ast::FlagsItem {
            kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
        }],
    };

    let translator_i = TranslatorI::new(&trans, "example_pattern");
    translator_i.set_flags(&ast_flags);
}

#[test]
fn test_set_flags_unicode() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let ast_flags = ast::Flags {
        items: vec![ast::FlagsItem {
            kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
        }],
    };

    let translator_i = TranslatorI::new(&trans, "example_pattern");
    translator_i.set_flags(&ast_flags);
}

#[test]
fn test_set_flags_multiple_flags() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
        ],
    };

    let translator_i = TranslatorI::new(&trans, "example_pattern");
    translator_i.set_flags(&ast_flags);
}

#[test]
fn test_set_flags_no_change() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: Some(false),
            multi_line: Some(false),
            dot_matches_new_line: Some(false),
            swap_greed: None,
            unicode: Some(true),
        }),
        allow_invalid_utf8: false,
    };
    let ast_flags = ast::Flags {
        items: vec![],
    };

    let translator_i = TranslatorI::new(&trans, "example_pattern");
    translator_i.set_flags(&ast_flags);
}

#[test]
fn test_set_flags_negation() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
        ],
    };

    let translator_i = TranslatorI::new(&trans, "example_pattern");
    translator_i.set_flags(&ast_flags);
}

