// Answer 0

#[test]
fn test_fmt_class_ascii_graph() {
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
    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: ast::Span {}, // Assuming Span can be instantiated like this
        kind: ast::ClassAsciiKind::Graph,
        negated: false,
    };

    let result = fmt_writer.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:graph:]");
} 

#[test]
fn test_fmt_class_ascii_graph_negated() {
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
    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: ast::Span {}, // Assuming Span can be instantiated like this
        kind: ast::ClassAsciiKind::Graph,
        negated: true,
    };

    let result = fmt_writer.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:^graph:]");
} 

#[test]
fn test_fmt_class_ascii_alpha() {
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
    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: ast::Span {}, // Assuming Span can be instantiated like this
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };

    let result = fmt_writer.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:alpha:]");
} 

#[test]
fn test_fmt_class_ascii_digit() {
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
    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: ast::Span {}, // Assuming Span can be instantiated like this
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };

    let result = fmt_writer.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:digit:]");
} 

#[test]
fn test_fmt_class_ascii_blank() {
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
    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: ast::Span {}, // Assuming Span can be instantiated like this
        kind: ast::ClassAsciiKind::Blank,
        negated: false,
    };

    let result = fmt_writer.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:blank:]");
}

