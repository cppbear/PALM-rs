// Answer 0

#[test]
#[should_panic]
fn test_hir_perl_byte_class_digit_unicode_true() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    translator.hir_perl_byte_class(&ast_class);
}

#[test]
#[should_panic]
fn test_hir_perl_byte_class_space_unicode_true() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Space,
        negated: true,
    };
    translator.hir_perl_byte_class(&ast_class);
}

#[test]
#[should_panic]
fn test_hir_perl_byte_class_word_unicode_true() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Word,
        negated: false,
    };
    translator.hir_perl_byte_class(&ast_class);
}

#[test]
fn test_hir_perl_byte_class_digit_unicode_false() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    translator.hir_perl_byte_class(&ast_class);
}

#[test]
fn test_hir_perl_byte_class_space_unicode_false() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Space,
        negated: true,
    };
    translator.hir_perl_byte_class(&ast_class);
}

#[test]
fn test_hir_perl_byte_class_word_unicode_false() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Word,
        negated: false,
    };
    translator.hir_perl_byte_class(&ast_class);
}

