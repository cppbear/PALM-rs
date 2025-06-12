// Answer 0

#[test]
fn test_fmt_class_perl_space_negated() {
    use std::fmt::Write;
    use ast::{ClassPerl, ClassPerlKind};

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let ast = ClassPerl {
        span: Span::default(), // Assuming Span can be defaulted
        kind: ClassPerlKind::Space,
        negated: true,
    };

    let result = writer.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "\\S");
}

#[test]
fn test_fmt_class_perl_space_not_negated() {
    use std::fmt::Write;
    use ast::{ClassPerl, ClassPerlKind};

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let ast = ClassPerl {
        span: Span::default(), // Assuming Span can be defaulted
        kind: ClassPerlKind::Space,
        negated: false,
    };

    let result = writer.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "\\s");
}

#[test]
fn test_fmt_class_perl_digit_negated() {
    use std::fmt::Write;
    use ast::{ClassPerl, ClassPerlKind};

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let ast = ClassPerl {
        span: Span::default(), // Assuming Span can be defaulted
        kind: ClassPerlKind::Digit,
        negated: true,
    };

    let result = writer.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "\\D");
}

#[test]
fn test_fmt_class_perl_digit_not_negated() {
    use std::fmt::Write;
    use ast::{ClassPerl, ClassPerlKind};

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let ast = ClassPerl {
        span: Span::default(), // Assuming Span can be defaulted
        kind: ClassPerlKind::Digit,
        negated: false,
    };

    let result = writer.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "\\d");
}

#[test]
fn test_fmt_class_perl_word_negated() {
    use std::fmt::Write;
    use ast::{ClassPerl, ClassPerlKind};

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let ast = ClassPerl {
        span: Span::default(), // Assuming Span can be defaulted
        kind: ClassPerlKind::Word,
        negated: true,
    };

    let result = writer.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "\\W");
}

#[test]
fn test_fmt_class_perl_word_not_negated() {
    use std::fmt::Write;
    use ast::{ClassPerl, ClassPerlKind};

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let ast = ClassPerl {
        span: Span::default(), // Assuming Span can be defaulted
        kind: ClassPerlKind::Word,
        negated: false,
    };

    let result = writer.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "\\w");
}

