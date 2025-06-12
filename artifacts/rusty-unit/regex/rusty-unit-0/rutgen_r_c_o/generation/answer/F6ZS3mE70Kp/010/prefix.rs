// Answer 0

#[test]
fn test_hir_unicode_class_unicode_not_allowed() {
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    
    let ast_class = ast::ClassUnicode {
        span: Span {
            start: Position(0),
            end: Position(10),
        },
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };
    
    let translator_i = TranslatorI::new(&translator, "test_pattern");
    let _ = translator_i.hir_unicode_class(&ast_class);
}

