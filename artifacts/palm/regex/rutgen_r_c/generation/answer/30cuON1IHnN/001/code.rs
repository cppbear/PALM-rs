// Answer 0

#[test]
fn test_flags_default() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "");

    let flags = translator_i.flags();
    assert_eq!(flags, Flags::default());
}

#[test]
fn test_flags_case_insensitive() {
    let mut flags = Flags::default();
    flags.case_insensitive = Some(true);

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "");

    let obtained_flags = translator_i.flags();
    assert_eq!(obtained_flags.case_insensitive, Some(true));
}

#[test]
fn test_flags_multi_line() {
    let mut flags = Flags::default();
    flags.multi_line = Some(true);

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "");

    let obtained_flags = translator_i.flags();
    assert_eq!(obtained_flags.multi_line, Some(true));
}

#[test]
fn test_flags_dot_matches_new_line() {
    let mut flags = Flags::default();
    flags.dot_matches_new_line = Some(true);

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "");

    let obtained_flags = translator_i.flags();
    assert_eq!(obtained_flags.dot_matches_new_line, Some(true));
}

#[test]
fn test_flags_swap_greed() {
    let mut flags = Flags::default();
    flags.swap_greed = Some(true);

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "");

    let obtained_flags = translator_i.flags();
    assert_eq!(obtained_flags.swap_greed, Some(true));
}

#[test]
fn test_flags_unicode() {
    let mut flags = Flags::default();
    flags.unicode = Some(true);

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "");

    let obtained_flags = translator_i.flags();
    assert_eq!(obtained_flags.unicode, Some(true));
}

