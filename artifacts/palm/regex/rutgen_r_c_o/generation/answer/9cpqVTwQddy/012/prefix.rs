// Answer 0

#[test]
fn test_fmt_class_ascii_print_not_negated() {
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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let ast = ClassAscii {
        span: Span {},  // Assume Span is defined appropriately
        kind: ClassAsciiKind::Print,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&ast);
}

