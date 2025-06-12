// Answer 0

#[test]
fn test_set_flags_case_insensitive_flag() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }

    let initial_flags = Flags {
        case_insensitive: Some(false),
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };

    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
        ],
    };

    let translator = DummyTranslator {
        flags: Cell::new(initial_flags.clone()),
    };

    let translator_ref = TranslatorI::new(&translator, "pattern");

    let old_flags = translator_ref.set_flags(&ast_flags);

    assert_eq!(old_flags.case_insensitive, Some(false));
    assert_eq!(translator.flags.get().case_insensitive, Some(true));
}

#[test]
fn test_set_flags_multi_line_flag() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }

    let initial_flags = Flags {
        case_insensitive: Some(true),
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };

    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
        ],
    };

    let translator = DummyTranslator {
        flags: Cell::new(initial_flags.clone()),
    };

    let translator_ref = TranslatorI::new(&translator, "pattern");

    let old_flags = translator_ref.set_flags(&ast_flags);

    assert_eq!(old_flags.multi_line, None);
    assert_eq!(translator.flags.get().multi_line, Some(true));
}

#[test]
fn test_set_flags_dot_matches_new_line_flag() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }

    let initial_flags = Flags {
        case_insensitive: None,
        multi_line: Some(true),
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };

    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) },
        ],
    };

    let translator = DummyTranslator {
        flags: Cell::new(initial_flags.clone()),
    };

    let translator_ref = TranslatorI::new(&translator, "pattern");

    let old_flags = translator_ref.set_flags(&ast_flags);

    assert_eq!(old_flags.dot_matches_new_line, None);
    assert_eq!(translator.flags.get().dot_matches_new_line, Some(true));
}

#[test]
fn test_set_flags_multiple_flags() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }

    let initial_flags = Flags {
        case_insensitive: Some(false),
        multi_line: Some(true),
        dot_matches_new_line: None,
        swap_greed: Some(false),
        unicode: None,
    };

    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
        ],
    };

    let translator = DummyTranslator {
        flags: Cell::new(initial_flags.clone()),
    };

    let translator_ref = TranslatorI::new(&translator, "pattern");

    let old_flags = translator_ref.set_flags(&ast_flags);

    assert_eq!(old_flags.case_insensitive, Some(false));
    assert_eq!(old_flags.multi_line, Some(true));
    assert_eq!(translator.flags.get().case_insensitive, Some(true));
    assert_eq!(translator.flags.get().multi_line, Some(true));
}

