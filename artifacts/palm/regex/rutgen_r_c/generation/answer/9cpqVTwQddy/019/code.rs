// Answer 0

#[test]
fn test_fmt_class_ascii_cntrl_negated() {
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
    
    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };
    let ast = ast::ClassAscii {
        span: Span, // Fill with appropriate Span value as needed
        kind: ast::ClassAsciiKind::Cntrl,
        negated: true,
    };
    
    let result = writer.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "[:^cntrl:]");
}

#[test]
fn test_fmt_class_ascii_cntrl_non_negated() {
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
    
    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };
    let ast = ast::ClassAscii {
        span: Span, // Fill with appropriate Span value as needed
        kind: ast::ClassAsciiKind::Cntrl,
        negated: false,
    };
    
    let result = writer.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "[:cntrl:]");
}

