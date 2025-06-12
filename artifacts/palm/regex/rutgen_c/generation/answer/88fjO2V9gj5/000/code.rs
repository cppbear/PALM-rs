// Answer 0

#[test]
fn test_translator_builder_new() {
    let builder = TranslatorBuilder {
        allow_invalid_utf8: false,
        flags: Flags {
            span: Span::default(),
            items: vec![],
        },
    };
    let translator = builder.build();
    assert_eq!(translator.allow_invalid_utf8, false);
    assert_eq!(translator.flags.get(), builder.flags);
}

#[test]
fn test_translator_builder_allow_invalid_utf8() {
    let mut builder = TranslatorBuilder {
        allow_invalid_utf8: false,
        flags: Flags {
            span: Span::default(),
            items: vec![],
        },
    };
    builder.allow_invalid_utf8(true);
    let translator = builder.build();
    assert_eq!(translator.allow_invalid_utf8, true);
}

#[test]
fn test_translator_builder_flags() {
    let flags = Flags {
        span: Span::default(),
        items: vec![],
    };
    let builder = TranslatorBuilder {
        allow_invalid_utf8: false,
        flags: flags.clone(),
    };
    let translator = builder.build();
    assert_eq!(translator.flags.get(), flags);
}

#[test]
fn test_translator_builder_with_case_insensitive() {
    let mut builder = TranslatorBuilder {
        allow_invalid_utf8: false,
        flags: Flags {
            span: Span::default(),
            items: vec![],
        },
    };
    builder.case_insensitive(true);
    let translator = builder.build();
    assert_eq!(translator.flags.get().case_insensitive, Some(true));
}

#[test]
fn test_translator_builder_with_multi_line() {
    let mut builder = TranslatorBuilder {
        allow_invalid_utf8: false,
        flags: Flags {
            span: Span::default(),
            items: vec![],
        },
    };
    builder.multi_line(true);
    let translator = builder.build();
    assert_eq!(translator.flags.get().multi_line, Some(true));
}

#[test]
fn test_translator_builder_with_dot_matches_new_line() {
    let mut builder = TranslatorBuilder {
        allow_invalid_utf8: false,
        flags: Flags {
            span: Span::default(),
            items: vec![],
        },
    };
    builder.dot_matches_new_line(true);
    let translator = builder.build();
    assert_eq!(translator.flags.get().dot_matches_new_line, Some(true));
}

#[test]
fn test_translator_builder_with_swap_greed() {
    let mut builder = TranslatorBuilder {
        allow_invalid_utf8: false,
        flags: Flags {
            span: Span::default(),
            items: vec![],
        },
    };
    builder.swap_greed(true);
    let translator = builder.build();
    assert_eq!(translator.flags.get().swap_greed, Some(true));
}

#[test]
fn test_translator_builder_with_unicode() {
    let mut builder = TranslatorBuilder {
        allow_invalid_utf8: false,
        flags: Flags {
            span: Span::default(),
            items: vec![],
        },
    };
    builder.unicode(true);
    let translator = builder.build();
    assert_eq!(translator.flags.get().unicode, Some(true));
}

