// Answer 0

#[test]
fn test_visit_class_set_item_post_ascii_word() {
    let translator = Translator::default();
    let pattern = "test";
    let mut translator_i = TranslatorI::new(&translator, pattern);
    let ast = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(10) },
        kind: ast::ClassAsciiKind::Word,
        negated: false,
    });
    translator_i.trans().flags.set(Flags { unicode: Some(false), ..Default::default() });
    translator_i.push(HirFrame::ClassBytes(ClassBytes::empty()));
    translator_i.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii_alpha() {
    let translator = Translator::default();
    let pattern = "test";
    let mut translator_i = TranslatorI::new(&translator, pattern);
    let ast = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(10) },
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    });
    translator_i.trans().flags.set(Flags { unicode: Some(false), ..Default::default() });
    translator_i.push(HirFrame::ClassBytes(ClassBytes::empty()));
    translator_i.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii_digit() {
    let translator = Translator::default();
    let pattern = "test";
    let mut translator_i = TranslatorI::new(&translator, pattern);
    let ast = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(10) },
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    });
    translator_i.trans().flags.set(Flags { unicode: Some(false), ..Default::default() });
    translator_i.push(HirFrame::ClassBytes(ClassBytes::empty()));
    translator_i.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii_lower() {
    let translator = Translator::default();
    let pattern = "test";
    let mut translator_i = TranslatorI::new(&translator, pattern);
    let ast = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(10) },
        kind: ast::ClassAsciiKind::Lower,
        negated: false,
    });
    translator_i.trans().flags.set(Flags { unicode: Some(false), ..Default::default() });
    translator_i.push(HirFrame::ClassBytes(ClassBytes::empty()));
    translator_i.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii_upper() {
    let translator = Translator::default();
    let pattern = "test";
    let mut translator_i = TranslatorI::new(&translator, pattern);
    let ast = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(10) },
        kind: ast::ClassAsciiKind::Upper,
        negated: false,
    });
    translator_i.trans().flags.set(Flags { unicode: Some(false), ..Default::default() });
    translator_i.push(HirFrame::ClassBytes(ClassBytes::empty()));
    translator_i.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii_space() {
    let translator = Translator::default();
    let pattern = "test";
    let mut translator_i = TranslatorI::new(&translator, pattern);
    let ast = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(10) },
        kind: ast::ClassAsciiKind::Space,
        negated: false,
    });
    translator_i.trans().flags.set(Flags { unicode: Some(false), ..Default::default() });
    translator_i.push(HirFrame::ClassBytes(ClassBytes::empty()));
    translator_i.visit_class_set_item_post(&ast).unwrap();
}

