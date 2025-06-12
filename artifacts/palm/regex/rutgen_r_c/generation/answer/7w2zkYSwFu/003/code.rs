// Answer 0

#[test]
fn test_fmt_class_set_binary_op_kind_intersection() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let kind = ast::ClassSetBinaryOpKind::Intersection;
    let result = writer_instance.fmt_class_set_binary_op_kind(&kind);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "&&");
}

#[test]
fn test_fmt_class_set_binary_op_kind_difference() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let kind = ast::ClassSetBinaryOpKind::Difference;
    let result = writer_instance.fmt_class_set_binary_op_kind(&kind);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "--");
}

#[test]
fn test_fmt_class_set_binary_op_kind_symmetric_difference() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let kind = ast::ClassSetBinaryOpKind::SymmetricDifference;
    let result = writer_instance.fmt_class_set_binary_op_kind(&kind);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "~~");
}

