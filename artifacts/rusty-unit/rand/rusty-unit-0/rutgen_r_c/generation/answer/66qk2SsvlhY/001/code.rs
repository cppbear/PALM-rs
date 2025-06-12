// Answer 0

#[test]
fn test_round_with_valid_state() {
    struct TestVec;

    impl ArithOps for TestVec {
        fn add(self, other: Self) -> Self {
            TestVec
        }
    }

    impl BitOps32 for TestVec {
        fn rotate_each_word_right16(self) -> Self {
            self
        }

        fn rotate_each_word_right20(self) -> Self {
            self
        }

        fn rotate_each_word_right24(self) -> Self {
            self
        }

        fn rotate_each_word_right25(self) -> Self {
            self
        }
    }

    let initial_state = State {
        a: TestVec,
        b: TestVec,
        c: TestVec,
        d: TestVec,
    };

    let result = round(initial_state);
    assert_eq!(std::any::type_name::<State<TestVec>>(), std::any::type_name_of_val(&result));
}

#[test]
fn test_round_with_edge_case_state() {
    struct EdgeCaseVec;

    impl ArithOps for EdgeCaseVec {
        fn add(self, other: Self) -> Self {
            EdgeCaseVec
        }
    }

    impl BitOps32 for EdgeCaseVec {
        fn rotate_each_word_right16(self) -> Self {
            self
        }

        fn rotate_each_word_right20(self) -> Self {
            self
        }

        fn rotate_each_word_right24(self) -> Self {
            self
        }

        fn rotate_each_word_right25(self) -> Self {
            self
        }
    }

    let edge_case_state = State {
        a: EdgeCaseVec,
        b: EdgeCaseVec,
        c: EdgeCaseVec,
        d: EdgeCaseVec,
    };

    let result = round(edge_case_state);
    assert_eq!(std::any::type_name::<State<EdgeCaseVec>>(), std::any::type_name_of_val(&result));
}

