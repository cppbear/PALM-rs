// Answer 0

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
    
    let assertion = Assertion {
        span: Span::default(),
        kind: AssertionKind::EndText,
    };

    writer.fmt_assertion(&assertion);
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
    
    let assertion = Assertion {
        span: Span::default(),
        kind: AssertionKind::WordBoundary,
    };

    writer.fmt_assertion(&assertion);
}

