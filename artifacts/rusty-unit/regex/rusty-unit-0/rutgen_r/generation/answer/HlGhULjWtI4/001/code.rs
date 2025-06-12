// Answer 0

#[test]
fn test_fmt_class_induct_binary_op_symmetric_difference() {
    struct TestClassSetBinaryOpKind {
        kind: ast::ClassSetBinaryOpKind,
    }

    struct TestClassInduct {
        variant: ClassInduct,
    }

    let binary_op = TestClassSetBinaryOpKind {
        kind: ast::ClassSetBinaryOpKind::SymmetricDifference,
    };

    let class_induct = TestClassInduct {
        variant: ClassInduct::BinaryOp(binary_op),
    };

    let mut output = Vec::new();
    let result = write!(&mut output, "{}", class_induct.variant);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "BinaryOp(SymmetricDifference)");
}

