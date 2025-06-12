// Answer 0

#[test]
fn test_fmt_class_induct_binary_op_difference() {
    struct FakeFormatter;

    impl fmt::Write for FakeFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut formatter = FakeFormatter;

    struct FakeClassSetBinaryOp {
        kind: ast::ClassSetBinaryOpKind,
    }

    struct FakeClassInduct {
        inner: ClassInduct,
    }

    let binary_op_difference = FakeClassSetBinaryOp {
        kind: ast::ClassSetBinaryOpKind::Difference,
    };

    let class_induct = FakeClassInduct {
        inner: ClassInduct::BinaryOp(binary_op_difference),
    };

    assert!(class_induct.fmt(&mut formatter).is_ok());
}

