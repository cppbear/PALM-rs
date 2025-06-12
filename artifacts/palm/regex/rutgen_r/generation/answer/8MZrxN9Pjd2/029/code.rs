// Answer 0

fn test_visit_pre_literal_unicode() -> fmt::Result {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn new() -> TestVisitor {
            TestVisitor {
                wtr: TestWriter { output: String::new() },
            }
        }

        fn write_literal_char(&mut self, _c: char) -> fmt::Result {
            self.wtr.write_str("c") // Just a test representation
        }
    }

    enum HirKind {
        Literal(Literal),
        Class(Class),
    }

    enum Literal {
        Unicode(char),
    }

    enum Class {
        Unicode(Vec<(char, char)>),
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut visitor = TestVisitor::new();
    let hir = Hir { kind: HirKind::Literal(Literal::Unicode('c')) };
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr.output, "c");
    Ok(())
}

fn test_visit_pre_literal_byte() -> fmt::Result {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn new() -> TestVisitor {
            TestVisitor {
                wtr: TestWriter { output: String::new() },
            }
        }

        fn write_literal_byte(&mut self, _b: u8) -> fmt::Result {
            self.wtr.write_str("b") // Just a test representation
        }
    }

    enum HirKind {
        Literal(Literal),
        Class(Class),
    }

    enum Literal {
        Byte(u8),
    }

    enum Class {
        Bytes(Vec<(u8, u8)>),
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut visitor = TestVisitor::new();
    let hir = Hir { kind: HirKind::Literal(Literal::Byte(1)) };
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr.output, "b");
    Ok(())
}

fn test_visit_pre_class_unicode() -> fmt::Result {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn new() -> TestVisitor {
            TestVisitor {
                wtr: TestWriter { output: String::new() },
            }
        }

        fn write_literal_char(&mut self, _c: char) -> fmt::Result {
            self.wtr.write_str("c") // Just a test representation
        }
    }

    enum HirKind {
        Class(Class),
    }

    enum Class {
        Unicode(Vec<(char, char)>),
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut visitor = TestVisitor::new();
    let hir = Hir { kind: HirKind::Class(Class::Unicode(vec![('a', 'a'), ('b', 'c')])) };
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr.output, "abc"); // Expect character span output
    Ok(())
}

fn test_visit_pre_class_bytes() -> fmt::Result {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn new() -> TestVisitor {
            TestVisitor {
                wtr: TestWriter { output: String::new() },
            }
        }

        fn write_literal_class_byte(&mut self, _b: u8) -> fmt::Result {
            self.wtr.write_str("b") // Just a test representation
        }
    }

    enum HirKind {
        Class(Class),
    }

    enum Class {
        Bytes(Vec<(u8, u8)>),
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut visitor = TestVisitor::new();
    let hir = Hir { kind: HirKind::Class(Class::Bytes(vec![(1, 1), (2, 3)])) };
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr.output, "bb"); // Expect byte span output
    Ok(())
}

