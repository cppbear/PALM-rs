// Answer 0

#[test]
fn test_fmt_class_ascii_blank_negated() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter {
        wtr: MockWriter,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                wtr: MockWriter::new(),
            }
        }
    }

    struct ClassAscii {
        kind: ast::ClassAsciiKind,
        negated: bool,
    }

    mod ast {
        pub enum ClassAsciiKind {
            Blank,
        }
    }

    let mut formatter = MockFormatter::new();
    let class_ascii = ClassAscii { kind: ast::ClassAsciiKind::Blank, negated: true };

    formatter.fmt_class_ascii(&class_ascii).unwrap();

    assert_eq!(formatter.wtr.output, "[:^blank:]");
}

#[test]
fn test_fmt_class_ascii_blank_non_negated() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter {
        wtr: MockWriter,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                wtr: MockWriter::new(),
            }
        }
    }

    struct ClassAscii {
        kind: ast::ClassAsciiKind,
        negated: bool,
    }

    mod ast {
        pub enum ClassAsciiKind {
            Blank,
        }
    }

    let mut formatter = MockFormatter::new();
    let class_ascii = ClassAscii { kind: ast::ClassAsciiKind::Blank, negated: false };

    formatter.fmt_class_ascii(&class_ascii).unwrap();

    assert_eq!(formatter.wtr.output, "[:blank:]");
}

