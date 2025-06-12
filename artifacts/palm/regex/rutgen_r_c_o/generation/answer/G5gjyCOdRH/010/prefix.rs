// Answer 0

#[test]
fn test_visit_class_set_item_post_ascii_valid() {
    use ast::{self, ClassAsciiKind, ClassSetItem};
    use hir::ClassUnicodeRange;

    let span = Span { start: Position::default(), end: Position::default() };
    let pattern = "a";
    
    let flags = Flags {
        unicode: Some(true),
        ..Flags::default()
    };
    
    let mut trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };

    let mut cls_unicode = ClassUnicode::empty();
    trans.stack.borrow_mut().push(HirFrame::ClassUnicode(cls_unicode));

    let x = ClassSetItem::Ascii(ast::ClassAscii {
        span,
        kind: ClassAsciiKind::Alnum,
        negated: false,
    });

    let mut translator_i = TranslatorI::new(&trans, pattern);
    translator_i.visit_class_set_item_post(&x).unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii_upper_bound() {
    use ast::{self, ClassAsciiKind, ClassSetItem};
    use hir::ClassUnicodeRange;

    let span = Span { start: Position::default(), end: Position::default() };
    let pattern = "z";

    let flags = Flags {
        unicode: Some(true),
        ..Flags::default()
    };

    let mut trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };

    let mut cls_unicode = ClassUnicode::empty();
    trans.stack.borrow_mut().push(HirFrame::ClassUnicode(cls_unicode));

    let x = ClassSetItem::Ascii(ast::ClassAscii {
        span,
        kind: ClassAsciiKind::Upper,
        negated: false,
    });

    let mut translator_i = TranslatorI::new(&trans, pattern);
    translator_i.visit_class_set_item_post(&x).unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii_blank() {
    use ast::{self, ClassAsciiKind, ClassSetItem};
    use hir::ClassUnicodeRange;

    let span = Span { start: Position::default(), end: Position::default() };
    let pattern = " ";

    let flags = Flags {
        unicode: Some(true),
        ..Flags::default()
    };

    let mut trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };

    let mut cls_unicode = ClassUnicode::empty();
    trans.stack.borrow_mut().push(HirFrame::ClassUnicode(cls_unicode));

    let x = ClassSetItem::Ascii(ast::ClassAscii {
        span,
        kind: ClassAsciiKind::Blank,
        negated: false,
    });

    let mut translator_i = TranslatorI::new(&trans, pattern);
    translator_i.visit_class_set_item_post(&x).unwrap();
}

