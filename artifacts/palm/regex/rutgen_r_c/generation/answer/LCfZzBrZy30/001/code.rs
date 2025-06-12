// Answer 0

#[test]
fn test_from_set_binary_op() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassSetBinaryOp {
        pub span: Span,
        pub kind: (),
        pub lhs: Box<ClassSet>,
        pub rhs: Box<ClassSet>,
    }
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ClassSet {
        Item(ClassSetItem),
        BinaryOp(ClassSetBinaryOp),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ClassSetItem {
        Empty(Span),
        Literal,
        Range,
        Ascii,
        Unicode,
        Perl,
        Bracketed(Box<ClassSet>),
        Union,
    }

    let span = Span;
    let lhs = Box::new(ClassSet::Item(ClassSetItem::Literal));
    let rhs = Box::new(ClassSet::Item(ClassSetItem::Empty(span.clone())));
    let binary_op = ClassSetBinaryOp {
        span,
        kind: (),
        lhs,
        rhs,
    };

    let ast = ClassSet::BinaryOp(binary_op);
    let result = ClassInduct::from_set(&ast);

    if let ClassInduct::BinaryOp(_) = result {
        // Success, the return type matches
    } else {
        panic!("Expected ClassInduct::BinaryOp, but received a different type.");
    }
}

