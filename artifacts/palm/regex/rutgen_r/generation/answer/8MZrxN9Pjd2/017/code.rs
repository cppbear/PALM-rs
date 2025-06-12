// Answer 0

fn test_visit_pre_empty() -> fmt::Result {
    struct TestWtr {
        output: String,
    }

    impl TestWtr {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWtr,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor { wtr: TestWtr { output: String::new() } }
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

    let mut visitor = TestVisitor::new();
    let hir = Hir::new(HirKind::Empty);
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr.output, "");
    Ok(())
}

fn test_visit_pre_word_boundary_unicode() -> fmt::Result {
    struct TestWtr {
        output: String,
    }

    impl TestWtr {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWtr,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor { wtr: TestWtr { output: String::new() } }
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

    let mut visitor = TestVisitor::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::Unicode));
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr.output, r"\b");
    Ok(())
}

fn test_visit_pre_word_boundary_unicode_negate() -> fmt::Result {
    struct TestWtr {
        output: String,
    }

    impl TestWtr {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWtr,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor { wtr: TestWtr { output: String::new() } }
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

    let mut visitor = TestVisitor::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate));
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr.output, r"\B");
    Ok(())
}

fn test_visit_pre_word_boundary_ascii() -> fmt::Result {
    struct TestWtr {
        output: String,
    }

    impl TestWtr {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWtr,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor { wtr: TestWtr { output: String::new() } }
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

    let mut visitor = TestVisitor::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::Ascii));
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr.output, "(?-u:\\b)");
    Ok(())
}

fn test_visit_pre_word_boundary_ascii_negate() -> fmt::Result {
    struct TestWtr {
        output: String,
    }

    impl TestWtr {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWtr,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor { wtr: TestWtr { output: String::new() } }
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

    let mut visitor = TestVisitor::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::AsciiNegate));
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr.output, "(?-u:\\B)");
    Ok(())
}

