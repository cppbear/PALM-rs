// Answer 0

#[test]
fn test_class_induct_binary_op_difference_debug() {
    struct DummySpan;
    struct DummyLiteral;
    struct DummyClassSet;
    struct DummyClassSetBinaryOpKind;
    
    impl ast::ClassSetBinaryOpKind {
        fn difference() -> Self {
            ast::ClassSetBinaryOpKind::Difference
        }
    }

    let span = DummySpan;
    let lhs = Box::new(DummyClassSet);
    let rhs = Box::new(DummyClassSet);
    let binary_op = ClassSetBinaryOp {
        span,
        kind: ast::ClassSetBinaryOpKind::difference(),
        lhs,
        rhs,
    };

    let induct = ClassInduct::BinaryOp(&binary_op);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "BinaryOp(Difference)");
}

