// Answer 0

#[test]
fn test_fmt_assertion_start_line() {
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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };
    let assertion = ast::Assertion { span: Span::new(0, 0), kind: ast::AssertionKind::StartLine };

    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(mock_writer.output, "^");
}

#[test]
fn test_fmt_assertion_end_line() {
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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };
    let assertion = ast::Assertion { span: Span::new(0, 0), kind: ast::AssertionKind::EndLine };

    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(mock_writer.output, "$");
}

#[test]
fn test_fmt_assertion_start_text() {
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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };
    let assertion = ast::Assertion { span: Span::new(0, 0), kind: ast::AssertionKind::StartText };

    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(mock_writer.output, r"\A");
}

#[test]
fn test_fmt_assertion_end_text() {
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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };
    let assertion = ast::Assertion { span: Span::new(0, 0), kind: ast::AssertionKind::EndText };

    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(mock_writer.output, r"\z");
}

#[test]
fn test_fmt_assertion_word_boundary() {
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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };
    let assertion = ast::Assertion { span: Span::new(0, 0), kind: ast::AssertionKind::WordBoundary };

    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(mock_writer.output, r"\b");
}

#[test]
fn test_fmt_assertion_not_word_boundary() {
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
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };
    let assertion = ast::Assertion { span: Span::new(0, 0), kind: ast::AssertionKind::NotWordBoundary };

    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(mock_writer.output, r"\B");
}

