// Answer 0

#[test]
fn test_fmt_class_perl_digit_negated() {
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
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span::new(0, 1), kind: ClassPerlKind::Digit, negated: true };
    writer_instance.fmt_class_perl(&class_perl).unwrap();

    assert_eq!(writer_instance.wtr.output, r"\D");
}

#[test]
fn test_fmt_class_perl_digit_non_negated() {
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
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span::new(0, 1), kind: ClassPerlKind::Digit, negated: false };
    writer_instance.fmt_class_perl(&class_perl).unwrap();

    assert_eq!(writer_instance.wtr.output, r"\d");
}

#[test]
fn test_fmt_class_perl_space_negated() {
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
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span::new(0, 1), kind: ClassPerlKind::Space, negated: true };
    writer_instance.fmt_class_perl(&class_perl).unwrap();

    assert_eq!(writer_instance.wtr.output, r"\S");
}

#[test]
fn test_fmt_class_perl_space_non_negated() {
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
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span::new(0, 1), kind: ClassPerlKind::Space, negated: false };
    writer_instance.fmt_class_perl(&class_perl).unwrap();

    assert_eq!(writer_instance.wtr.output, r"\s");
}

#[test]
fn test_fmt_class_perl_word_negated() {
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
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span::new(0, 1), kind: ClassPerlKind::Word, negated: true };
    writer_instance.fmt_class_perl(&class_perl).unwrap();

    assert_eq!(writer_instance.wtr.output, r"\W");
}

#[test]
fn test_fmt_class_perl_word_non_negated() {
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
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span::new(0, 1), kind: ClassPerlKind::Word, negated: false };
    writer_instance.fmt_class_perl(&class_perl).unwrap();

    assert_eq!(writer_instance.wtr.output, r"\w");
}

