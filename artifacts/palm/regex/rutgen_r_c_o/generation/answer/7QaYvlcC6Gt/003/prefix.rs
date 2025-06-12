// Answer 0

#[test]
fn test_hir_perl_byte_class_word_not_negated() {
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };

    let ast_class = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Word,
        negated: false,
    };

    let translator_i = TranslatorI::new(&translator, "pattern");
    let class_bytes = translator_i.hir_perl_byte_class(&ast_class);
}

