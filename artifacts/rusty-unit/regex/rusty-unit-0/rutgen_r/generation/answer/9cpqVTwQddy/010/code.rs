// Answer 0

#[test]
fn test_fmt_class_ascii_punct_non_negated() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: MockWriter,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: MockWriter::new(),
            }
        }
    }

    enum ClassAsciiKind {
        Punct,
        // other kinds omitted for brevity
    }

    struct ClassAscii {
        kind: ClassAsciiKind,
        negated: bool,
    }

    let mut formatter = Formatter::new();
    let ast = ClassAscii {
        kind: ClassAsciiKind::Punct,
        negated: false,
    };

    let result = formatter.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "[:punct:]");
}

#[test]
fn test_fmt_class_ascii_punct_negated() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: MockWriter,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: MockWriter::new(),
            }
        }
    }

    enum ClassAsciiKind {
        Punct,
        // other kinds omitted for brevity
    }

    struct ClassAscii {
        kind: ClassAsciiKind,
        negated: bool,
    }

    let mut formatter = Formatter::new();
    let ast = ClassAscii {
        kind: ClassAsciiKind::Punct,
        negated: true,
    };

    let result = formatter.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "[:^punct:]");
}

