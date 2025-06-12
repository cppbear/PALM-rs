// Answer 0

#[test]
fn test_visit_class_set_item_post_bracketed_non_unicode_empty() {
    use ast::{ClassSetItem, ClassBracketed};
    use hir::{ClassBytes, ClassBytesRange};
    
    let mut translator = Translator { 
        stack: RefCell::new(vec![]), 
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }), 
        allow_invalid_utf8: false 
    };
    
    let span = Span { start: Position(0), end: Position(1) };
    let negated = false;
    let class_set_item = ClassSetItem::Bracketed(ClassBracketed { span, negated });
    
    translator.push(HirFrame::ClassBytes(ClassBytes::empty()));
    
    let result = translator.visit_class_set_item_post(&class_set_item);
    
    assert!(result.is_ok());

    let frame = translator.pop().unwrap();
    if let HirFrame::ClassBytes(cls) = frame {
        assert!(cls.ranges().is_empty());
    } else {
        panic!("Expected a ClassBytes frame.");
    }
}

#[test]
#[should_panic]
fn test_visit_class_set_item_post_bracketed_non_unicode_invalid_utf8() {
    use ast::{ClassSetItem, ClassBracketed};
    use hir::{ClassBytes, ClassBytesRange};
    
    let mut translator = Translator { 
        stack: RefCell::new(vec![]), 
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }), 
        allow_invalid_utf8: false 
    };

    let span = Span { start: Position(0), end: Position(1) };
    let negated = false;
    let class_set_item = ClassSetItem::Bracketed(ClassBracketed { span, negated });
    
    // Push a non-empty class to simulate an invalid UTF-8 situation
    let mut cls = ClassBytes::empty();
    cls.push(ClassBytesRange::new(0x80, 0xFF)); // non-ASCII range
    translator.push(HirFrame::ClassBytes(cls));

    let _ = translator.visit_class_set_item_post(&class_set_item);
}

