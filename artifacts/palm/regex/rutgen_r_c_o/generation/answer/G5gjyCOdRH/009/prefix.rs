// Answer 0

#[test]
fn test_visit_class_set_item_post_ascii_alnum() {
    let mut translator = Translator {
        stack: RefCell::new(vec![HirFrame::ClassUnicode(ClassUnicode::empty())]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let ast_item = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(1) },
        kind: ast::ClassAsciiKind::Alnum,
        negated: false,
    });
    translator.visit_class_set_item_post(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii_alpha() {
    let mut translator = Translator {
        stack: RefCell::new(vec![HirFrame::ClassUnicode(ClassUnicode::empty())]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let ast_item = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(1) },
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    });
    translator.visit_class_set_item_post(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii_space() {
    let mut translator = Translator {
        stack: RefCell::new(vec![HirFrame::ClassUnicode(ClassUnicode::empty())]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let ast_item = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(1) },
        kind: ast::ClassAsciiKind::Space,
        negated: false,
    });
    translator.visit_class_set_item_post(&ast_item).unwrap();
}

