// Answer 0

#[test]
fn test_fmt_class_perl_digit_not_negated() {
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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let ast = ClassPerl {
        span: Span {},
        kind: ClassPerlKind::Digit,
        negated: false,
    };

    writer_ref.fmt_class_perl(&ast);
}

