// Answer 0

fn test_visit_pre_unicode_literal() -> fmt::Result {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_literal_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    use regex_syntax::hir::{Hir, HirKind, Literal};

    let mut visitor = TestVisitor {
        wtr: TestWriter { output: String::new() },
    };

    let hir = Hir::literal(Literal::Unicode('a'));
    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "a");

    Ok(())
}

fn test_visit_pre_byte_literal() -> fmt::Result {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
            Ok(())
        }
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    use regex_syntax::hir::{Hir, HirKind, Literal};

    let mut visitor = TestVisitor {
        wtr: TestWriter { output: String::new() },
    };

    let hir = Hir::literal(Literal::Byte(b'a'));
    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "a");

    Ok(())
}

fn test_visit_pre_empty() -> fmt::Result {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    use regex_syntax::hir::{Hir, HirKind};

    let mut visitor = TestVisitor {
        wtr: TestWriter { output: String::new() },
    };

    let hir = Hir::empty();
    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());

    Ok(())
}

