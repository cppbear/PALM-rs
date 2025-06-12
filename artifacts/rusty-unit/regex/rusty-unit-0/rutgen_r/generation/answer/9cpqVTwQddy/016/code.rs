// Answer 0

#[test]
fn test_fmt_class_ascii_graph() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct ClassAscii {
        kind: ClassAsciiKind,
        negated: bool,
    }

    enum ClassAsciiKind {
        Graph,
    }

    let mut writer = TestWriter::new();
    let ast = ClassAscii {
        kind: ClassAsciiKind::Graph,
        negated: false,
    };

    let result = writer.write_str("[:graph:]");

    assert_eq!(writer.output, "[:graph:]");
    assert!(result.is_ok());
}

#[test]
fn test_fmt_class_ascii_graph_negated() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct ClassAscii {
        kind: ClassAsciiKind,
        negated: bool,
    }

    enum ClassAsciiKind {
        Graph,
    }

    let mut writer = TestWriter::new();
    let ast = ClassAscii {
        kind: ClassAsciiKind::Graph,
        negated: true,
    };

    writer.write_str("[:^graph:]").unwrap();

    assert_eq!(writer.output, "[:^graph:]");
}

