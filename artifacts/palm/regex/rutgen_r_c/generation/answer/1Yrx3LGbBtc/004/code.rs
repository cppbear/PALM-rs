// Answer 0

#[test]
fn test_fmt_class_perl_digit_non_negated() {
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

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };

    let class_perl = ClassPerl {
        span: Span {},  // Assuming Span is defined elsewhere in the context
        kind: ClassPerlKind::Digit,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let result = writer_instance.fmt_class_perl(&class_perl);

    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, r"\d");
}

#[test]
fn test_fmt_class_perl_digit_negated() {
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

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };

    let class_perl = ClassPerl {
        span: Span {},
        kind: ClassPerlKind::Digit,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let result = writer_instance.fmt_class_perl(&class_perl);

    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, r"\D");
}

#[test]
fn test_fmt_class_perl_space_non_negated() {
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

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };

    let class_perl = ClassPerl {
        span: Span {},
        kind: ClassPerlKind::Space,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let result = writer_instance.fmt_class_perl(&class_perl);

    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, r"\s");
}

#[test]
fn test_fmt_class_perl_space_negated() {
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

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };

    let class_perl = ClassPerl {
        span: Span {},
        kind: ClassPerlKind::Space,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let result = writer_instance.fmt_class_perl(&class_perl);

    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, r"\S");
}

#[test]
fn test_fmt_class_perl_word_non_negated() {
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

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };

    let class_perl = ClassPerl {
        span: Span {},
        kind: ClassPerlKind::Word,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let result = writer_instance.fmt_class_perl(&class_perl);

    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, r"\w");
}

#[test]
fn test_fmt_class_perl_word_negated() {
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

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };

    let class_perl = ClassPerl {
        span: Span {},
        kind: ClassPerlKind::Word,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let result = writer_instance.fmt_class_perl(&class_perl);

    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, r"\W");
}

