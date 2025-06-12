// Answer 0

#[test]
fn test_child_union() {
    struct TestFrame;
    
    impl From<TestFrame> for ClassFrame {
        fn from(_: TestFrame) -> ClassFrame {
            ClassFrame::Union { head: 'a' }
        }
    }

    let frame: ClassFrame = TestFrame.into();
    let induct = frame.child();
    match induct {
        ClassInduct::Item(head) => assert_eq!(head, 'a'),
        _ => panic!("Expected ClassInduct::Item"),
    }
}

#[test]
fn test_child_binary() {
    struct TestFrame;
    
    impl From<TestFrame> for ClassFrame {
        fn from(_: TestFrame) -> ClassFrame {
            ClassFrame::Binary { op: '+' }
        }
    }

    let frame: ClassFrame = TestFrame.into();
    let induct = frame.child();
    match induct {
        ClassInduct::BinaryOp(op) => assert_eq!(op, '+'),
        _ => panic!("Expected ClassInduct::BinaryOp"),
    }
}

#[test]
fn test_child_binary_lhs() {
    struct TestFrame;
    
    impl From<TestFrame> for ClassFrame {
        fn from(_: TestFrame) -> ClassFrame {
            ClassFrame::BinaryLHS { lhs: 'x' }
        }
    }

    let frame: ClassFrame = TestFrame.into();
    let induct = frame.child();
    match induct {
        ClassInduct::Set(set) => assert_eq!(set, 'x'), // Assuming from_set returns a char.
        _ => panic!("Expected ClassInduct::Set"),
    }
}

#[test]
fn test_child_binary_rhs() {
    struct TestFrame;
    
    impl From<TestFrame> for ClassFrame {
        fn from(_: TestFrame) -> ClassFrame {
            ClassFrame::BinaryRHS { rhs: 'y' }
        }
    }

    let frame: ClassFrame = TestFrame.into();
    let induct = frame.child();
    match induct {
        ClassInduct::Set(set) => assert_eq!(set, 'y'), // Assuming from_set returns a char.
        _ => panic!("Expected ClassInduct::Set"),
    }
}

