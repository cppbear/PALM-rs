// Answer 0

#[test]
fn test_fmt_assertion_start_line() {
    use std::fmt::Write;
    use ast::{AssertionKind, Assertion};

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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: mock_writer };

    let assertion = Assertion { span: Span::default(), kind: AssertionKind::StartLine };
    writer.fmt_assertion(&assertion).unwrap();

    assert_eq!(writer.wtr.output, "^");
}

#[test]
fn test_fmt_assertion_end_line() {
    use std::fmt::Write;
    use ast::{AssertionKind, Assertion};

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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: mock_writer };

    let assertion = Assertion { span: Span::default(), kind: AssertionKind::EndLine };
    writer.fmt_assertion(&assertion).unwrap();

    assert_eq!(writer.wtr.output, "$");
}

#[test]
fn test_fmt_assertion_start_text() {
    use std::fmt::Write;
    use ast::{AssertionKind, Assertion};

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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: mock_writer };

    let assertion = Assertion { span: Span::default(), kind: AssertionKind::StartText };
    writer.fmt_assertion(&assertion).unwrap();

    assert_eq!(writer.wtr.output, r"\A");
}

#[test]
fn test_fmt_assertion_end_text() {
    use std::fmt::Write;
    use ast::{AssertionKind, Assertion};

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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: mock_writer };

    let assertion = Assertion { span: Span::default(), kind: AssertionKind::EndText };
    writer.fmt_assertion(&assertion).unwrap();

    assert_eq!(writer.wtr.output, r"\z");
}

#[test]
fn test_fmt_assertion_word_boundary() {
    use std::fmt::Write;
    use ast::{AssertionKind, Assertion};

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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: mock_writer };

    let assertion = Assertion { span: Span::default(), kind: AssertionKind::WordBoundary };
    writer.fmt_assertion(&assertion).unwrap();

    assert_eq!(writer.wtr.output, r"\b");
}

#[test]
fn test_fmt_assertion_not_word_boundary() {
    use std::fmt::Write;
    use ast::{AssertionKind, Assertion};

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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: mock_writer };

    let assertion = Assertion { span: Span::default(), kind: AssertionKind::NotWordBoundary };
    writer.fmt_assertion(&assertion).unwrap();

    assert_eq!(writer.wtr.output, r"\B");
}

