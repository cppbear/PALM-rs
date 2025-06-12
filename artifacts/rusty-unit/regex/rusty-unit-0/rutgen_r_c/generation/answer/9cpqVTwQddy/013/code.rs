// Answer 0

#[test]
fn test_fmt_class_ascii_lower_negated() {
    struct MockPrinter {
        output: String,
    }

    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut printer };

    let ast = ClassAscii {
        span: Span::default(), // Provide a suitable default for the `span`
        kind: ClassAsciiKind::Lower,
        negated: true,
    };

    writer.fmt_class_ascii(&ast).unwrap();
    assert_eq!(printer.output, "[:^lower:]");
}

#[test]
fn test_fmt_class_ascii_lower_not_negated() {
    struct MockPrinter {
        output: String,
    }

    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut printer };

    let ast = ClassAscii {
        span: Span::default(), // Provide a suitable default for the `span`
        kind: ClassAsciiKind::Lower,
        negated: false,
    };

    writer.fmt_class_ascii(&ast).unwrap();
    assert_eq!(printer.output, "[:lower:]");
}

