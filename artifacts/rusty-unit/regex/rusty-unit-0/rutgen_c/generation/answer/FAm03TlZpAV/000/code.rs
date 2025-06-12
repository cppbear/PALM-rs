// Answer 0

#[test]
fn test_fmt_class_bracketed_pre_negated() {
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
    let class_bracketed = ClassBracketed { span: Default::default(), negated: true, kind: Default::default() };

    writer.fmt_class_bracketed_pre(&class_bracketed).unwrap();

    assert_eq!(test_writer.output, "[^");
}

#[test]
fn test_fmt_class_bracketed_pre_non_negated() {
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
    let class_bracketed = ClassBracketed { span: Default::default(), negated: false, kind: Default::default() };

    writer.fmt_class_bracketed_pre(&class_bracketed).unwrap();

    assert_eq!(test_writer.output, "[");
}

