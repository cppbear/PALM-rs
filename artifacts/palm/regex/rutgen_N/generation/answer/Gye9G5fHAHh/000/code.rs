// Answer 0

#[test]
fn test_visit_class_set_binary_op_post_intersection_unicode() {
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
        fn unwrap_class_unicode(&self) -> MockHirFrame {
            self.clone()
        }

        fn intersect(&mut self, _rhs: &MockHirFrame) {
            self.kind = "IntersectedUnicode".to_string();
        }

        fn union(&mut self, _lhs: &MockHirFrame) {
            // No-op for the purpose of this test
        }
    }

    struct MockVisitor {
        flags: MockFlags,
        stack: Vec<MockHirFrame>,
    }

    impl MockVisitor {
        fn new(flags: MockFlags) -> Self {
            Self {
                flags,
                stack: vec![],
            }
        }

        fn push(&mut self, frame: MockHirFrame) {
            self.stack.push(frame);
        }

        fn pop(&mut self) -> Option<MockHirFrame> {
            self.stack.pop()
        }

        fn flags(&self) -> &MockFlags {
            &self.flags
        }
    }

    let mut visitor = MockVisitor::new(MockFlags { unicode: true, case_insensitive: false });
    visitor.push(MockHirFrame { kind: "ClassA".to_string() });
    visitor.push(MockHirFrame { kind: "ClassB".to_string() });
    visitor.push(MockHirFrame { kind: "ClassC".to_string() });

    let op = ast::ClassSetBinaryOp {
        kind: ast::ClassSetBinaryOpKind::Intersection,
    };

    visitor.visit_class_set_binary_op_post(&op).unwrap();
}

#[test]
fn test_visit_class_set_binary_op_post_difference_bytes() {
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
        fn unwrap_class_bytes(&self) -> MockHirFrame {
            self.clone()
        }

        fn difference(&mut self, _rhs: &MockHirFrame) {
            self.kind = "DifferencedBytes".to_string();
        }

        fn union(&mut self, _lhs: &MockHirFrame) {
            // No-op for the purpose of this test
        }
    }

    struct MockVisitor {
        flags: MockFlags,
        stack: Vec<MockHirFrame>,
    }

    impl MockVisitor {
        fn new(flags: MockFlags) -> Self {
            Self {
                flags,
                stack: vec![],
            }
        }

        fn push(&mut self, frame: MockHirFrame) {
            self.stack.push(frame);
        }

        fn pop(&mut self) -> Option<MockHirFrame> {
            self.stack.pop()
        }

        fn flags(&self) -> &MockFlags {
            &self.flags
        }
    }

    let mut visitor = MockVisitor::new(MockFlags { unicode: false, case_insensitive: false });
    visitor.push(MockHirFrame { kind: "ClassX".to_string() });
    visitor.push(MockHirFrame { kind: "ClassY".to_string() });
    visitor.push(MockHirFrame { kind: "ClassZ".to_string() });

    let op = ast::ClassSetBinaryOp {
        kind: ast::ClassSetBinaryOpKind::Difference,
    };

    visitor.visit_class_set_binary_op_post(&op).unwrap();
}

