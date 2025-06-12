// Answer 0

fn test_visit_pre_word_boundary_unicode_negate() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl<'a> TestVisitor<'a> {
        fn new(wtr: &'a mut TestWriter) -> Self {
            TestVisitor { wtr }
        }
        
        fn write_literal_char(&mut self, _c: char) -> fmt::Result {
            Ok(())
        }

        fn write_literal_byte(&mut self, _b: u8) -> fmt::Result {
            Ok(())
        }

        fn write_literal_class_byte(&mut self, _b: u8) -> fmt::Result {
            Ok(())
        }
    }

    enum HirKind {
        WordBoundary(WordBoundary),
        // Other variants omitted for brevity
    }

    enum WordBoundary {
        UnicodeNegate,
        // Other variants omitted for brevity
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    // Arrange
    let mut writer = TestWriter::new();
    let mut visitor = TestVisitor::new(&mut writer);
    let hir = Hir {
        kind: HirKind::WordBoundary(WordBoundary::UnicodeNegate),
    };

    // Act
    let result = visitor.visit_pre(&hir);

    // Assert
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, r"\B");
}

fn test_visit_pre_literal_unicode() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl<'a> TestVisitor<'a> {
        fn new(wtr: &'a mut TestWriter) -> Self {
            TestVisitor { wtr }
        }
        
        fn write_literal_char(&mut self, c: char) -> fmt::Result {
            self.wtr.write_str(&c.to_string())
        }

        fn write_literal_byte(&mut self, _b: u8) -> fmt::Result {
            Ok(())
        }

        fn write_literal_class_byte(&mut self, _b: u8) -> fmt::Result {
            Ok(())
        }
    }

    enum HirKind {
        Literal(Literal),
        // Other variants omitted for brevity
    }

    enum Literal {
        Unicode(char),
        // Other variants omitted for brevity
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    // Arrange
    let mut writer = TestWriter::new();
    let mut visitor = TestVisitor::new(&mut writer);
    let hir = Hir {
        kind: HirKind::Literal(Literal::Unicode('a')),
    };

    // Act
    let result = visitor.visit_pre(&hir);

    // Assert
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "a");
}

fn test_visit_pre_literal_byte() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl<'a> TestVisitor<'a> {
        fn new(wtr: &'a mut TestWriter) -> Self {
            TestVisitor { wtr }
        }
        
        fn write_literal_char(&mut self, _c: char) -> fmt::Result {
            Ok(())
        }

        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            self.wtr.output.push(b as char);
            Ok(())
        }

        fn write_literal_class_byte(&mut self, _b: u8) -> fmt::Result {
            Ok(())
        }
    }

    enum HirKind {
        Literal(Literal),
        // Other variants omitted for brevity
    }

    enum Literal {
        Byte(u8),
        // Other variants omitted for brevity
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    // Arrange
    let mut writer = TestWriter::new();
    let mut visitor = TestVisitor::new(&mut writer);
    let hir = Hir {
        kind: HirKind::Literal(Literal::Byte(b'a')),
    };

    // Act
    let result = visitor.visit_pre(&hir);

    // Assert
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "a");
}

