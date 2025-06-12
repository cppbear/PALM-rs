// Answer 0

#[test]
fn test_fmt_class_unicode_not_equal() {
    let mut buffer = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut buffer,
    };
    
    let ast = ast::ClassUnicode {
        span: ast::Span::default(),
        negated: true,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::NotEqual,
            name: "category".to_string(),
            value: "value".to_string(),
        },
    };
    
    writer.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_equal() {
    let mut buffer = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut buffer,
    };
    
    let ast = ast::ClassUnicode {
        span: ast::Span::default(),
        negated: true,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Equal,
            name: "category".to_string(),
            value: "value".to_string(),
        },
    };
    
    writer.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_colon() {
    let mut buffer = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut buffer,
    };
    
    let ast = ast::ClassUnicode {
        span: ast::Span::default(),
        negated: true,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Colon,
            name: "category".to_string(),
            value: "value".to_string(),
        },
    };
    
    writer.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_named() {
    let mut buffer = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut buffer,
    };

    let ast = ast::ClassUnicode {
        span: ast::Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::Named("category".to_string()),
    };

    writer.fmt_class_unicode(&ast);
}

