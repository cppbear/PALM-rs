// Answer 0

#[test]
fn test_fmt_class_perl_digit_non_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let test_class_perl = ClassPerl {
        span: Span { /* initialize with valid span */ },
        kind: ClassPerlKind::Digit,
        negated: false,
    };

    let result = writer.fmt_class_perl(&test_class_perl);

    assert!(result.is_ok());
    assert_eq!(mock_writer.output, r"\d");
}

#[test]
fn test_fmt_class_perl_digit_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let test_class_perl = ClassPerl {
        span: Span { /* initialize with valid span */ },
        kind: ClassPerlKind::Digit,
        negated: true,
    };

    let result = writer.fmt_class_perl(&test_class_perl);

    assert!(result.is_ok());
    assert_eq!(mock_writer.output, r"\D");
}

#[test]
fn test_fmt_class_perl_space_non_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let test_class_perl = ClassPerl {
        span: Span { /* initialize with valid span */ },
        kind: ClassPerlKind::Space,
        negated: false,
    };

    let result = writer.fmt_class_perl(&test_class_perl);

    assert!(result.is_ok());
    assert_eq!(mock_writer.output, r"\s");
}

#[test]
fn test_fmt_class_perl_space_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let test_class_perl = ClassPerl {
        span: Span { /* initialize with valid span */ },
        kind: ClassPerlKind::Space,
        negated: true,
    };

    let result = writer.fmt_class_perl(&test_class_perl);

    assert!(result.is_ok());
    assert_eq!(mock_writer.output, r"\S");
}

#[test]
fn test_fmt_class_perl_word_non_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let test_class_perl = ClassPerl {
        span: Span { /* initialize with valid span */ },
        kind: ClassPerlKind::Word,
        negated: false,
    };

    let result = writer.fmt_class_perl(&test_class_perl);

    assert!(result.is_ok());
    assert_eq!(mock_writer.output, r"\w");
}

#[test]
fn test_fmt_class_perl_word_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let test_class_perl = ClassPerl {
        span: Span { /* initialize with valid span */ },
        kind: ClassPerlKind::Word,
        negated: true,
    };

    let result = writer.fmt_class_perl(&test_class_perl);

    assert!(result.is_ok());
    assert_eq!(mock_writer.output, r"\W");
}

