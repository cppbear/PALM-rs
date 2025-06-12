// Answer 0

#[test]
fn test_fmt_class_ascii_upper() {
    use std::fmt::Write; // For the write_str method
    use ast::{ClassAscii, ClassAsciiKind, Span}; // Replace with actual imports as needed

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
    let ast = ClassAscii {
        span: Span {}, // Use appropriate initialization
        kind: ClassAsciiKind::Upper,
        negated: false,
    };

    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    writer.fmt_class_ascii(&ast).unwrap();

    assert_eq!(printer.output, "[:upper:]");
}

#[test]
fn test_fmt_class_ascii_upper_negated() {
    use std::fmt::Write; // For the write_str method
    use ast::{ClassAscii, ClassAsciiKind, Span}; // Replace with actual imports as needed

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
    let ast = ClassAscii {
        span: Span {}, // Use appropriate initialization
        kind: ClassAsciiKind::Upper,
        negated: true,
    };

    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    writer.fmt_class_ascii(&ast).unwrap();

    assert_eq!(printer.output, "[:^upper:]");
}

#[test]
fn test_fmt_class_ascii_lower() {
    use std::fmt::Write; // For the write_str method
    use ast::{ClassAscii, ClassAsciiKind, Span}; // Replace with actual imports as needed

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
    let ast = ClassAscii {
        span: Span {}, // Use appropriate initialization
        kind: ClassAsciiKind::Lower,
        negated: false,
    };

    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    writer.fmt_class_ascii(&ast).unwrap();

    assert_eq!(printer.output, "[:lower:]");
}

#[test]
fn test_fmt_class_ascii_lower_negated() {
    use std::fmt::Write; // For the write_str method
    use ast::{ClassAscii, ClassAsciiKind, Span}; // Replace with actual imports as needed

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
    let ast = ClassAscii {
        span: Span {}, // Use appropriate initialization
        kind: ClassAsciiKind::Lower,
        negated: true,
    };

    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    writer.fmt_class_ascii(&ast).unwrap();

    assert_eq!(printer.output, "[:^lower:]");
}

