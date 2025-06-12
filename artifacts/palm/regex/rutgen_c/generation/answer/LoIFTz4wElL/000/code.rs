// Answer 0

#[test]
fn test_fmt_class_bracketed_post() {
    use std::fmt::Write;

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
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: mock_writer };

    let class_bracketed = ClassBracketed { span: Span::default(), negated: false, kind: ClassSet::default() };
    
    let result = writer.fmt_class_bracketed_post(&class_bracketed);
    
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "]");
}

