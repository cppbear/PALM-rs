// Answer 0

#[test]
fn test_fmt_class_set_binary_op_kind_intersection() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::ClassSetBinaryOpKind::Intersection;
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = writer_instance.fmt_class_set_binary_op_kind(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "&&");
}

#[test]
fn test_fmt_class_set_binary_op_kind_difference() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::ClassSetBinaryOpKind::Difference;
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = writer_instance.fmt_class_set_binary_op_kind(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "--");
}

#[test]
fn test_fmt_class_set_binary_op_kind_symmetric_difference() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::ClassSetBinaryOpKind::SymmetricDifference;
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = writer_instance.fmt_class_set_binary_op_kind(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "~~");
}

