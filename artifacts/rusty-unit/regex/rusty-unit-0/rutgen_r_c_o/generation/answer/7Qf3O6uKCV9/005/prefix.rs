// Answer 0

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
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let assertion = Assertion {
        span: Span::new(0, 1),
        kind: ast::AssertionKind::EndLine,
    };

    writer.fmt_assertion(&assertion);
}

