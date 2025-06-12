// Answer 0

#[test]
fn test_hir_unicode_class_with_named_value() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let pattern = "\\p{age=1}";
    let span = Span { start: Position(0), end: Position(13) };
    let ast_class = ast::ClassUnicode {
        span,
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            name: "age",
            value: "1",
        },
    };
    let translator_i = TranslatorI::new(&translator, pattern);
    let _ = translator_i.hir_unicode_class(&ast_class);
}

