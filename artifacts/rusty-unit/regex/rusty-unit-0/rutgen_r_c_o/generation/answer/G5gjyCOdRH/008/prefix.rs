// Answer 0

#[test]
fn test_visit_class_set_item_post_unicode() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    
    let unicode_class = ast::ClassUnicode {
        span: Span { start: Position(0), end: Position(10) },
        kind: ast::ClassUnicodeKind::OneLetter('A'),
        negated: false,
    };

    let class_set_item = ast::ClassSetItem::Unicode(unicode_class);
    translator.push(HirFrame::ClassUnicode(hir::ClassUnicode::empty()));

    translator.visit_class_set_item_post(&class_set_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_unicode_negated() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let unicode_class = ast::ClassUnicode {
        span: Span { start: Position(0), end: Position(10) },
        kind: ast::ClassUnicodeKind::Named("Greek"),
        negated: true,
    };

    let class_set_item = ast::ClassSetItem::Unicode(unicode_class);
    translator.push(HirFrame::ClassUnicode(hir::ClassUnicode::empty()));

    translator.visit_class_set_item_post(&class_set_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_unicode_with_valid_class() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };

    let unicode_class = ast::ClassUnicode {
        span: Span { start: Position(0), end: Position(10) },
        kind: ast::ClassUnicodeKind::NamedValue { name: "script".into(), value: "Latin".into() },
        negated: false,
    };

    let class_set_item = ast::ClassSetItem::Unicode(unicode_class);
    translator.push(HirFrame::ClassUnicode(hir::ClassUnicode::empty()));

    translator.visit_class_set_item_post(&class_set_item).unwrap();
}

