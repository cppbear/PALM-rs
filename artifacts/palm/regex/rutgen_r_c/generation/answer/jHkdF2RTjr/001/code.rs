// Answer 0

#[test]
fn test_empty_literals() {
    struct TestArgs {
        flag_literal_limit: usize,
        flag_class_limit: usize,
    }

    impl TestArgs {
        fn empty_literals(&self) -> Literals {
            let mut lits = Literals::empty();
            lits.set_limit_size(self.flag_literal_limit);
            lits.set_limit_class(self.flag_class_limit);
            lits
        }
    }

    // Test Case 1: Default limits
    let args1 = TestArgs {
        flag_literal_limit: 250,
        flag_class_limit: 10,
    };
    let literals1 = args1.empty_literals();
    assert_eq!(literals1.size_limit(), 250);
    assert_eq!(literals1.class_limit(), 10);

    // Test Case 2: Minimum limits
    let args2 = TestArgs {
        flag_literal_limit: 1,
        flag_class_limit: 1,
    };
    let literals2 = args2.empty_literals();
    assert_eq!(literals2.size_limit(), 1);
    assert_eq!(literals2.class_limit(), 1);

    // Test Case 3: Upper boundary limits
    let args3 = TestArgs {
        flag_literal_limit: usize::MAX,
        flag_class_limit: usize::MAX,
    };
    let literals3 = args3.empty_literals();
    assert_eq!(literals3.size_limit(), usize::MAX);
    assert_eq!(literals3.class_limit(), usize::MAX);
}

