// Answer 0

#[test]
fn test_fmt_class_ascii_space_negated() {
    use std::fmt::Write; // Importing necessary trait for fmt::Result
    use ast::{ClassAscii, ClassAsciiKind};

    struct MockPrinter {
        output: String,
    }

    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockPrinter { output: String::new() };
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let ast = ClassAscii {
        span: Span::default(), // Assume some default span exists
        kind: ClassAsciiKind::Space,
        negated: true,
    };

    let result = writer.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "[:^space:]");
}

#[test]
fn test_fmt_class_ascii_space_non_negated() {
    use std::fmt::Write; // Importing necessary trait for fmt::Result
    use ast::{ClassAscii, ClassAsciiKind};

    struct MockPrinter {
        output: String,
    }

    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockPrinter { output: String::new() };
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    let ast = ClassAscii {
        span: Span::default(), // Assume some default span exists
        kind: ClassAsciiKind::Space,
        negated: false,
    };

    let result = writer.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "[:space:]");
}

