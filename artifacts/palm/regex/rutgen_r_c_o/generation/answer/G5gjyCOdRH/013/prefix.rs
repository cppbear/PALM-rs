// Answer 0

#[test]
fn test_visit_class_set_item_post_ascii_invalid_span() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let pattern = "test_pattern";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let span = Span { start: Position::default(), end: Position::default() };
    let literal = ast::Literal { span: span.clone(), kind: ast::LiteralKind::Char, c: 'c' };
    let ascii = ast::ClassAscii { span: span.clone(), kind: ast::ClassAsciiKind::Digit, negated: true };
    let class_set_item = ast::ClassSetItem::Ascii(ascii);
    
    visitor.pop = Some(HirFrame::ClassBytes(ClassBytes::empty()));
    visitor.visit_class_set_item_post(&class_set_item).ok();
}

#[test]
fn test_visit_class_set_item_post_ascii_negated() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: true,
    };
    let pattern = "test_pattern";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let span = Span { start: Position::default(), end: Position::default() };
    let ascii = ast::ClassAscii { span: span.clone(), kind: ast::ClassAsciiKind::Space, negated: false };
    let class_set_item = ast::ClassSetItem::Ascii(ascii);
    
    visitor.pop = Some(HirFrame::ClassBytes(ClassBytes::empty()));
    visitor.visit_class_set_item_post(&class_set_item).ok();
}

#[test]
#[should_panic]
fn test_visit_class_set_item_post_ascii_panic_empty_case() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let pattern = "test_pattern";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let span = Span { start: Position::default(), end: Position::default() };
    let ascii = ast::ClassAscii { span: span.clone(), kind: ast::ClassAsciiKind::Digit, negated: false };
    let class_set_item = ast::ClassSetItem::Ascii(ascii);
    
    visitor.visit_class_set_item_post(&class_set_item).ok();
}

#[test]
fn test_visit_class_set_item_post_ascii_case() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: true,
    };
    let pattern = "test_pattern";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let span = Span { start: Position::default(), end: Position::default() };
    let ascii = ast::ClassAscii { span: span.clone(), kind: ast::ClassAsciiKind::Alpha, negated: false };
    let class_set_item = ast::ClassSetItem::Ascii(ascii);
    
    visitor.pop = Some(HirFrame::ClassBytes(ClassBytes::empty()));
    visitor.visit_class_set_item_post(&class_set_item).ok();
}

