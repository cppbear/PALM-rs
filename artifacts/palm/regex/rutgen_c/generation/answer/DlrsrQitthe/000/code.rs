// Answer 0

#[test]
fn test_from_bracketed_literal() {
    struct Span {
        start: usize,
        end: usize,
    }
    
    struct Literal(char);
    
    let span = Span { start: 0, end: 1 };
    let class_set_item = ClassSetItem::Literal(Literal('a'));
    let class_set = ClassSet::Item(class_set_item);
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: class_set,
    };
    
    let induct = ClassInduct::from_bracketed(&class_bracketed);
    match induct {
        ClassInduct::Item(item) => match item {
            ClassSetItem::Literal(Literal(c)) => assert_eq!(c, 'a'),
            _ => panic!("Expected a Literal"),
        },
        _ => panic!("Expected ClassInduct::Item"),
    }
}

#[test]
fn test_from_bracketed_range() {
    struct Span {
        start: usize,
        end: usize,
    }
    
    struct ClassSetRange {
        start: char,
        end: char,
    }
    
    let span = Span { start: 0, end: 5 };
    let class_set_item = ClassSetItem::Range(ClassSetRange { start: 'a', end: 'z' });
    let class_set = ClassSet::Item(class_set_item);
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: class_set,
    };
    
    let induct = ClassInduct::from_bracketed(&class_bracketed);
    match induct {
        ClassInduct::Item(item) => match item {
            ClassSetItem::Range(ClassSetRange { start, end }) => {
                assert_eq!(start, 'a');
                assert_eq!(end, 'z');
            },
            _ => panic!("Expected a Range"),
        },
        _ => panic!("Expected ClassInduct::Item"),
    }
}

#[test]
fn test_from_bracketed_negated() {
    struct Span {
        start: usize,
        end: usize,
    }
    
    let span = Span { start: 0, end: 1 };
    let class_set_item = ClassSetItem::Bracketed(Box::new(ClassBracketed {
        span,
        negated: true,
        kind: ClassSet::Item(ClassSetItem::Literal(Literal('b'))),
    }));
    let class_set = ClassSet::Item(class_set_item);
    let class_bracketed = ClassBracketed {
        span,
        negated: true,
        kind: class_set,
    };
    
    let induct = ClassInduct::from_bracketed(&class_bracketed);
    match induct {
        ClassInduct::Item(item) => match item {
            ClassSetItem::Bracketed(inner) => {
                assert!(inner.negated);
                if let ClassSet::Item(ClassSetItem::Literal(Literal(c))) = inner.kind {
                    assert_eq!(c, 'b');
                } else {
                    panic!("Expected a Literal in nested Bracketed");
                }
            },
            _ => panic!("Expected a Bracketed class set"),
        },
        _ => panic!("Expected ClassInduct::Item"),
    }
}

