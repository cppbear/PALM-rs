// Answer 0

#[test]
fn test_visit_class_set_binary_op_post_sym_difference() {
    struct MockFlags {
        unicode: bool,
        case_insensitive: bool,
    }

    impl MockFlags {
        fn unicode(&self) -> bool {
            self.unicode
        }

        fn case_insensitive(&self) -> bool {
            self.case_insensitive
        }
    }

    struct MockHirFrame {
        kind: String,
    }

    impl MockHirFrame {
        fn unwrap_class_bytes(self) -> Self {
            self
        }

        fn union(&mut self, _other: &Self) {}

        fn intersect(&mut self, _other: &Self) {}

        fn difference(&mut self, _other: &Self) {}

        fn symmetric_difference(&mut self, _other: &Self) {}
    }

    struct MockSelf {
        flags: MockFlags,
        stack: Vec<MockHirFrame>,
    }

    impl MockSelf {
        fn flags(&self) -> &MockFlags {
            &self.flags
        }

        fn pop(&mut self) -> Option<Result<MockHirFrame, ()>> {
            self.stack.pop().map(|v| Ok(v))
        }

        fn push(&mut self, frame: MockHirFrame) {
            self.stack.push(frame);
        }
    }

    let mut mock_self = MockSelf {
        flags: MockFlags {
            unicode: false,
            case_insensitive: false,
        },
        stack: vec![
            MockHirFrame { kind: String::from("class1") },
            MockHirFrame { kind: String::from("class2") },
            MockHirFrame { kind: String::from("class3") },
        ],
    };

    let op = ast::ClassSetBinaryOp {
        kind: ast::ClassSetBinaryOpKind::SymmetricDifference,
    };

    let result = mock_self.visit_class_set_binary_op_post(&op);
    assert!(result.is_ok());
}

