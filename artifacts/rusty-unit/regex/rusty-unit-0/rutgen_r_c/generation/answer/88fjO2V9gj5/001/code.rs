// Answer 0

#[test]
fn test_translator_builder_default() {
    let builder = TranslatorBuilder {
        allow_invalid_utf8: false,
        flags: Flags {
            span: Span::default(), // Assuming `Span::default()` exists
            items: vec![],
        },
    };
    let translator = builder.build();

    assert_eq!(translator.stack.borrow().len(), 0);
    assert_eq!(translator.allow_invalid_utf8, false);
}

#[test]
fn test_translator_builder_allow_invalid_utf8() {
    let builder = TranslatorBuilder {
        allow_invalid_utf8: true,
        flags: Flags {
            span: Span::default(), // Assuming `Span::default()` exists
            items: vec![],
        },
    };
    let translator = builder.build();

    assert_eq!(translator.stack.borrow().len(), 0);
    assert_eq!(translator.allow_invalid_utf8, true);
}

#[test]
fn test_translator_builder_with_flags() {
    let flags = Flags {
        span: Span::default(), // Assuming `Span::default()` exists
        items: vec![], // Here you could add specific flag items if needed
    };

    let builder = TranslatorBuilder {
        allow_invalid_utf8: false,
        flags: flags.clone(),
    };
    let translator = builder.build();

    assert_eq!(translator.stack.borrow().len(), 0);
    assert_eq!(translator.flags.get(), flags);
    assert_eq!(translator.allow_invalid_utf8, false);
}

#[test]
fn test_translator_builder_multiple_flags() {
    let flags = Flags {
        span: Span::default(), // Assuming `Span::default()` exists
        items: vec![
            FlagsItem::CaseInsensitive,
            FlagsItem::MultiLine,
        ],
    };

    let builder = TranslatorBuilder {
        allow_invalid_utf8: true,
        flags: flags.clone(),
    };
    let translator = builder.build();

    assert_eq!(translator.stack.borrow().len(), 0);
    assert_eq!(translator.flags.get(), flags);
    assert_eq!(translator.allow_invalid_utf8, true);
}

