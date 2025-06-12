// Answer 0

#[test]
fn test_hir_perl_byte_class_digit() {
    struct SimpleTranslator;

    let translator = SimpleTranslator;
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let result = translator.hir_perl_byte_class(&ast_class);
    assert!(result.is_all_ascii());
}

#[test]
fn test_hir_perl_byte_class_space() {
    struct SimpleTranslator;

    let translator = SimpleTranslator;
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };

    let result = translator.hir_perl_byte_class(&ast_class);
    assert!(result.is_all_ascii());
}

#[test]
fn test_hir_perl_byte_class_word() {
    struct SimpleTranslator;

    let translator = SimpleTranslator;
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };

    let result = translator.hir_perl_byte_class(&ast_class);
    assert!(result.is_all_ascii());
}

#[test]
fn test_hir_perl_byte_class_negated_digit() {
    struct SimpleTranslator;

    let translator = SimpleTranslator;
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };

    let result = translator.hir_perl_byte_class(&ast_class);
    assert!(!result.is_all_ascii());
}

#[test]
fn test_hir_perl_byte_class_negated_space() {
    struct SimpleTranslator;

    let translator = SimpleTranslator;
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };

    let result = translator.hir_perl_byte_class(&ast_class);
    assert!(!result.is_all_ascii());
}

#[test]
fn test_hir_perl_byte_class_negated_word() {
    struct SimpleTranslator;

    let translator = SimpleTranslator;
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Word,
        negated: true,
    };

    let result = translator.hir_perl_byte_class(&ast_class);
    assert!(!result.is_all_ascii());
}

