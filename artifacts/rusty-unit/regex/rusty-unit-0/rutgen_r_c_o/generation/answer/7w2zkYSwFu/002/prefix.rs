// Answer 0

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

    let mut test_writer = TestWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut test_writer };
    let ast = ast::ClassSetBinaryOpKind::Difference;

    writer.fmt_class_set_binary_op_kind(&ast);
}

