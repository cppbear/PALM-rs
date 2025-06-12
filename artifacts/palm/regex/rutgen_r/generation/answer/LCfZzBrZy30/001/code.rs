// Answer 0

#[test]
fn test_from_set_binary_op() {
    struct TestOp;
    
    impl TestOp {
        fn new() -> Self {
            TestOp {}
        }
    }

    enum ClassSet {
        Item(Box<TestItem>),
        BinaryOp(Box<TestOp>),
    }

    struct TestItem;

    enum ClassInduct<'a> {
        Item(&'a TestItem),
        BinaryOp(&'a TestOp),
    }

    fn from_set<'a>(ast: &'a ClassSet) -> ClassInduct<'a> {
        match *ast {
            ClassSet::Item(ref item) => ClassInduct::Item(item),
            ClassSet::BinaryOp(ref op) => ClassInduct::BinaryOp(op),
        }
    }

    let op = Box::new(TestOp::new());
    let ast = ClassSet::BinaryOp(op);
    match from_set(&ast) {
        ClassInduct::BinaryOp(_) => {},
        _ => panic!("Expected ClassInduct::BinaryOp"),
    }
}

