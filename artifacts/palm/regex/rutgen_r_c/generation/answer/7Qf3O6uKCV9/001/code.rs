// Answer 0

#[test]
fn test_fmt_assertion_not_word_boundary() {
    use std::fmt::Write;
    use ast::{Assertion, AssertionKind};

    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let assertion = Assertion { span: Span::default(), kind: AssertionKind::NotWordBoundary };
    let mut printer = Printer { _priv: () };
    let mut writer_ref = Writer { printer: &mut printer, wtr: writer };

    let result = writer_ref.fmt_assertion(&assertion);

    assert!(result.is_ok());
    assert_eq!(writer_ref.wtr.output, r"\B");
}

