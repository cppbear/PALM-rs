// Answer 0

#[test]
fn test_fmt_class_unicode_one_letter() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };
    
    writer.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_named_value_equal() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Equal,
            name: String::from("category"),
            value: String::from("someValue"),
        },
    };
    
    writer.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_named_value_colon() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Colon,
            name: String::from("category"),
            value: String::from("anotherValue"),
        },
    };
    
    writer.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_named_value_not_equal() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::NotEqual,
            name: String::from("category"),
            value: String::from("someValue"),
        },
    };
    
    writer.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_named() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::Named(String::from("category")),
    };
    
    writer.fmt_class_unicode(&ast);
}

