// Answer 0

#[test]
fn test_visit_class_set_binary_op_in_intersection() {
    use ast::{ClassSet, ClassSetBinaryOp, ClassSetBinaryOpKind};
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let lhs = Box::new(ClassSet {});
    let rhs = Box::new(ClassSet {});
    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::Intersection,
        lhs,
        rhs,
    };

    let result = writer.visit_class_set_binary_op_in(&op);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "&&");
}

#[test]
fn test_visit_class_set_binary_op_in_difference() {
    use ast::{ClassSet, ClassSetBinaryOp, ClassSetBinaryOpKind};
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let lhs = Box::new(ClassSet {});
    let rhs = Box::new(ClassSet {});
    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::Difference,
        lhs,
        rhs,
    };

    let result = writer.visit_class_set_binary_op_in(&op);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "--");
}

#[test]
fn test_visit_class_set_binary_op_in_symmetric_difference() {
    use ast::{ClassSet, ClassSetBinaryOp, ClassSetBinaryOpKind};
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let lhs = Box::new(ClassSet {});
    let rhs = Box::new(ClassSet {});
    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::SymmetricDifference,
        lhs,
        rhs,
    };

    let result = writer.visit_class_set_binary_op_in(&op);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "~~");
}

