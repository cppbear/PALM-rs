// Answer 0

#[test]
fn test_visit_class_set_binary_op_in_intersection() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: MockWriter { output: String::new() },
    };

    let ast = ast::ClassSetBinaryOp {
        span: Span {}, // initialize properly based on actual Span struct
        kind: ast::ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(ClassSet {}), // initialize properly based on actual ClassSet struct
        rhs: Box::new(ClassSet {}), // initialize properly based on actual ClassSet struct
    };

    let result = writer.visit_class_set_binary_op_in(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "&&");
}

#[test]
fn test_visit_class_set_binary_op_in_difference() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: MockWriter { output: String::new() },
    };

    let ast = ast::ClassSetBinaryOp {
        span: Span {}, // initialize properly based on actual Span struct
        kind: ast::ClassSetBinaryOpKind::Difference,
        lhs: Box::new(ClassSet {}), // initialize properly based on actual ClassSet struct
        rhs: Box::new(ClassSet {}), // initialize properly based on actual ClassSet struct
    };

    let result = writer.visit_class_set_binary_op_in(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "--");
}

#[test]
fn test_visit_class_set_binary_op_in_symmetric_difference() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: MockWriter { output: String::new() },
    };

    let ast = ast::ClassSetBinaryOp {
        span: Span {}, // initialize properly based on actual Span struct
        kind: ast::ClassSetBinaryOpKind::SymmetricDifference,
        lhs: Box::new(ClassSet {}), // initialize properly based on actual ClassSet struct
        rhs: Box::new(ClassSet {}), // initialize properly based on actual ClassSet struct
    };

    let result = writer.visit_class_set_binary_op_in(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "~~");
}

