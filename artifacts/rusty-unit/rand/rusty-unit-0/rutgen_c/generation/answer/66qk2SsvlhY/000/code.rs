// Answer 0

#[test]
fn test_round_with_u32() {
    struct TestArithOps;
    impl ArithOps for TestArithOps {
        fn add(self, other: Self) -> Self {
            TestArithOps
        }
    }

    impl BitOps32 for TestArithOps {
        fn rotate_each_word_right16(self) -> Self {
            TestArithOps
        }
        
        fn rotate_each_word_right20(self) -> Self {
            TestArithOps
        }
        
        fn rotate_each_word_right24(self) -> Self {
            TestArithOps
        }
        
        fn rotate_each_word_right25(self) -> Self {
            TestArithOps
        }
    }
    
    let initial_state = State {
        a: TestArithOps,
        b: TestArithOps,
        c: TestArithOps,
        d: TestArithOps,
    };

    let result_state = round(initial_state);
    // Add assertions here to validate expected behavior of the `result_state`
}

#[test]
fn test_round_with_other_type() {
    struct AnotherArithOps;
    impl ArithOps for AnotherArithOps {
        fn add(self, other: Self) -> Self {
            AnotherArithOps
        }
    }

    impl BitOps32 for AnotherArithOps {
        fn rotate_each_word_right16(self) -> Self {
            AnotherArithOps
        }
        
        fn rotate_each_word_right20(self) -> Self {
            AnotherArithOps
        }
        
        fn rotate_each_word_right24(self) -> Self {
            AnotherArithOps
        }
        
        fn rotate_each_word_right25(self) -> Self {
            AnotherArithOps
        }
    }

    let initial_state = State {
        a: AnotherArithOps,
        b: AnotherArithOps,
        c: AnotherArithOps,
        d: AnotherArithOps,
    };

    let result_state = round(initial_state);
    // Add assertions here to validate expected behavior of the `result_state`
}

