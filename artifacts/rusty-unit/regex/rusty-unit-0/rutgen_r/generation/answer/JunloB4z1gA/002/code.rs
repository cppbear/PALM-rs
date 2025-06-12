// Answer 0

#[test]
fn test_class_set_item_bracketed_span() {
    struct Span {
        start: usize,
        end: usize,
    }
    
    struct Bracketed {
        span: Span,
    }
    
    enum ClassSetItem {
        Empty(Span),
        Literal { span: Span },
        Range { span: Span },
        Ascii { span: Span },
        Perl { span: Span },
        Unicode { span: Span },
        Bracketed(Bracketed),
        Union { span: Span },
    }

    let bracketed_item = ClassSetItem::Bracketed(Bracketed {
        span: Span { start: 5, end: 10 },
    });

    match bracketed_item {
        ClassSetItem::Bracketed(ref x) => {
            assert_eq!(x.span.start, 5);
            assert_eq!(x.span.end, 10);
        },
        _ => panic!("Expected ClassSetItem::Bracketed"),
    }
}

