// Answer 0

#[test]
fn test_visit_class_set_item_post_ascii_unicode() {
    use ast::{ClassSetItem, ClassAscii, ClassAsciiKind, Flags};
    use hir::{ClassUnicode, ClassUnicodeRange};
    use std::cell::RefCell;
    
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags { unicode: Some(true), ..Default::default() }),
        allow_invalid_utf8: false,
    };

    let mut translator_i = TranslatorI::new(&translator, "");

    let ascii_item = ClassSetItem::Ascii(ClassAscii {
        span: Span { start: Position(0), end: Position(1) },
        kind: ClassAsciiKind::Alnum,
        negated: false,
    });

    translator_i.push(HirFrame::ClassUnicode(ClassUnicode::empty()));
    
    let result = translator_i.visit_class_set_item_post(&ascii_item);
    
    assert_eq!(result, Ok(()));
    
    // Verify that the correct ranges were added to the class.
    let cls = translator_i.pop().unwrap().unwrap_class_unicode();
    let ranges: Vec<(char, char)> = ascii_class(&ClassAsciiKind::Alnum).to_vec();
    
    for (s, e) in ranges {
        assert!(cls.ranges().iter().any(|r| r.start() == s && r.end() == e));
    }
}

