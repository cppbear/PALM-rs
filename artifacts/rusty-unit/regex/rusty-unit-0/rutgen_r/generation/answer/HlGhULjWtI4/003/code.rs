// Answer 0

#[test]
fn test_fmt_binary_op_intersection() {
    struct TestClassSetBinaryOpKind {
        kind: ast::ClassSetBinaryOpKind,
    }

    struct TestClassInduct {
        item: Option<TestClassSetItem>,
        binary_op: Option<TestClassSetBinaryOp>,
    }

    struct TestClassSetBinaryOp {
        kind: ast::ClassSetBinaryOpKind,
    }

    enum TestClassSetItem {
        Empty,
        Literal,
        Range,
        Ascii,
        Perl,
        Unicode,
        Bracketed,
        Union,
    }

    impl fmt::Display for TestClassInduct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let x = match self.binary_op {
                Some(ref bo) if bo.kind == ast::ClassSetBinaryOpKind::Intersection => {
                    "BinaryOp(Intersection)"
                }
                Some(ref bo) if bo.kind == ast::ClassSetBinaryOpKind::Difference => {
                    "BinaryOp(Difference)"
                }
                Some(ref bo) if bo.kind == ast::ClassSetBinaryOpKind::SymmetricDifference => {
                    "BinaryOp(SymmetricDifference)"
                }
                None => return Err(fmt::Error),
            };
            write!(f, "{}", x)
        }
    }

    let binary_op = TestClassSetBinaryOp {
        kind: ast::ClassSetBinaryOpKind::Intersection,
    };

    let class_induct = TestClassInduct {
        item: None,
        binary_op: Some(binary_op),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", class_induct);
    assert!(result.is_ok());
    assert_eq!(output, "BinaryOp(Intersection)");
}

