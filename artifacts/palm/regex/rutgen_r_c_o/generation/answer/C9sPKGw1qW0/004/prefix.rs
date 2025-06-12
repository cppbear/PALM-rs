// Answer 0

#[test]
fn test_visit_class_set_item_post_unicode_literal() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let unicode_literal = ast::ClassSetItem::Unicode(ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::OneLetter('a'),
    });
    writer.visit_class_set_item_post(&unicode_literal).unwrap();
}

#[test]
fn test_visit_class_set_item_post_unicode_negated_literal() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let unicode_literal = ast::ClassSetItem::Unicode(ast::ClassUnicode {
        span: Span::default(),
        negated: true,
        kind: ClassUnicodeKind::OneLetter('A'),
    });
    writer.visit_class_set_item_post(&unicode_literal).unwrap();
}

#[test]
fn test_visit_class_set_item_post_unicode_named() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let unicode_named = ast::ClassSetItem::Unicode(ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::Named("Scx"),
    });
    writer.visit_class_set_item_post(&unicode_named).unwrap();
}

#[test]
fn test_visit_class_set_item_post_unicode_named_value_equal() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let unicode_named_value = ast::ClassSetItem::Unicode(ast::ClassUnicode {
        span: Span::default(),
        negated: true,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::Equal,
            name: "Scx".to_string(),
            value: "Katakana".to_string(),
        },
    });
    writer.visit_class_set_item_post(&unicode_named_value).unwrap();
}

#[test]
fn test_visit_class_set_item_post_unicode_named_value_not_equal() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let unicode_named_value = ast::ClassSetItem::Unicode(ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: "Scx".to_string(),
            value: "Hiragana".to_string(),
        },
    });
    writer.visit_class_set_item_post(&unicode_named_value).unwrap();
}

#[test]
fn test_visit_class_set_item_post_unicode_range() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let unicode_range = ast::ClassSetItem::Unicode(ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::Range(ast::ClassSetRange {
            span: Span::default(),
            start: ast::Literal {
                span: Span::default(),
                kind: ast::LiteralKind::Verbatim,
                c: '\u{00A0}',
            },
            end: ast::Literal {
                span: Span::default(),
                kind: ast::LiteralKind::Verbatim,
                c: '\u{FFFF}',
            },
        }),
    });
    writer.visit_class_set_item_post(&unicode_range).unwrap();
}

