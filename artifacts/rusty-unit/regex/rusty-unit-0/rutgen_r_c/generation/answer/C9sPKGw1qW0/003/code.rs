// Answer 0

#[test]
fn test_visit_class_set_item_post_perl_digit_not_negated() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    });

    let result = writer.visit_class_set_item_post(&ast);
    assert_eq!(result.unwrap(), ());
    assert_eq!(output, r"\d");
}

#[test]
fn test_visit_class_set_item_post_perl_digit_negated() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    });

    let result = writer.visit_class_set_item_post(&ast);
    assert_eq!(result.unwrap(), ());
    assert_eq!(output, r"\D");
}

#[test]
fn test_visit_class_set_item_post_perl_word_not_negated() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Word,
        negated: false,
    });

    let result = writer.visit_class_set_item_post(&ast);
    assert_eq!(result.unwrap(), ());
    assert_eq!(output, r"\w");
}

#[test]
fn test_visit_class_set_item_post_perl_word_negated() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Word,
        negated: true,
    });

    let result = writer.visit_class_set_item_post(&ast);
    assert_eq!(result.unwrap(), ());
    assert_eq!(output, r"\W");
}

#[test]
fn test_visit_class_set_item_post_perl_space_not_negated() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: false,
    });

    let result = writer.visit_class_set_item_post(&ast);
    assert_eq!(result.unwrap(), ());
    assert_eq!(output, r"\s");
}

#[test]
fn test_visit_class_set_item_post_perl_space_negated() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: true,
    });

    let result = writer.visit_class_set_item_post(&ast);
    assert_eq!(result.unwrap(), ());
    assert_eq!(output, r"\S");
}

