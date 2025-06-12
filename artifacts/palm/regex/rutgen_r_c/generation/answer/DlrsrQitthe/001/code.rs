// Answer 0

#[test]
fn test_from_bracketed_normal() {
    struct Span {
        start: usize,
        end: usize,
    }
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ClassSet {
        Item(ClassSetItem),
        BinaryOp(ClassSetBinaryOp),
    }
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ClassSetItem {
        Empty(Span),
        Literal(char),
        Range(ClassSetRange),
        Ascii(ClassAscii),
        Unicode(ClassUnicode),
        Perl(ClassPerl),
        Bracketed(Box<ClassBracketed>),
        Union(ClassSetUnion),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassBracketed {
        span: Span,
        negated: bool,
        kind: ClassSet,
    }

    struct ClassSetRange {
        start: char,
        end: char,
    }

    struct ClassAscii;

    struct ClassUnicode;

    struct ClassPerl;

    struct ClassSetUnion;

    let span = Span { start: 0, end: 5 };
    let item = ClassSetItem::Literal('a');
    let class_set = ClassSet::Item(item);
    let bracketed = ClassBracketed {
        span,
        negated: false,
        kind: class_set,
    };
    
    let induct = ClassInduct::from_bracketed(&bracketed);
    
    match induct {
        ClassInduct::Item(_) => assert!(true),
        _ => assert!(false, "Expected ClassInduct::Item"),
    }
}

#[test]
fn test_from_bracketed_negated() {
    struct Span {
        start: usize,
        end: usize,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ClassSet {
        Item(ClassSetItem),
        BinaryOp(ClassSetBinaryOp),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ClassSetItem {
        Empty(Span),
        Literal(char),
        Range(ClassSetRange),
        Ascii(ClassAscii),
        Unicode(ClassUnicode),
        Perl(ClassPerl),
        Bracketed(Box<ClassBracketed>),
        Union(ClassSetUnion),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassBracketed {
        span: Span,
        negated: bool,
        kind: ClassSet,
    }

    struct ClassSetRange {
        start: char,
        end: char,
    }

    struct ClassAscii;

    struct ClassUnicode;

    struct ClassPerl;

    struct ClassSetUnion;

    let span = Span { start: 0, end: 5 };
    let item = ClassSetItem::Bracketed(Box::new(ClassBracketed {
        span: Span { start: 1, end: 3 },
        negated: true,
        kind: ClassSet::Item(ClassSetItem::Empty(span.clone())),
    }));
    
    let class_set = ClassSet::Item(item);
    let bracketed = ClassBracketed {
        span,
        negated: true,
        kind: class_set,
    };

    let induct = ClassInduct::from_bracketed(&bracketed);
    
    match induct {
        ClassInduct::Item(_) => assert!(true),
        _ => assert!(false, "Expected ClassInduct::Item"),
    }
}

