// Answer 0

fn test_visit_pre_anchor_start_line() -> fmt::Result {
    struct TestVisitor {
        wtr: String,
    }

    impl TestVisitor {
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
        Anchor(Anchor),
    }

    enum Anchor {
        StartLine,
        EndLine,
        StartText,
        EndText,
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut visitor = TestVisitor { wtr: String::new() };
    let hir = Hir { kind: HirKind::Anchor(Anchor::StartLine) };
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr, "(?m:^)");
    Ok(())
}

fn test_visit_pre_anchor_end_line() -> fmt::Result {
    struct TestVisitor {
        wtr: String,
    }

    impl TestVisitor {
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
        Anchor(Anchor),
    }

    enum Anchor {
        StartLine,
        EndLine,
        StartText,
        EndText,
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut visitor = TestVisitor { wtr: String::new() };
    let hir = Hir { kind: HirKind::Anchor(Anchor::EndLine) };
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr, "(?m:$)");
    Ok(())
}

fn test_visit_pre_literal_unicode() -> fmt::Result {
    struct TestVisitor {
        wtr: String,
    }

    impl TestVisitor {
        fn write_literal_char(&mut self, c: char) -> fmt::Result {
            self.wtr.push(c);
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
        Literal(Literal),
    }

    enum Literal {
        Unicode(char),
        Byte(u8),
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut visitor = TestVisitor { wtr: String::new() };
    let hir = Hir { kind: HirKind::Literal(Literal::Unicode('a')) };
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr, "a");
    Ok(())
}

fn test_visit_pre_literal_byte() -> fmt::Result {
    struct TestVisitor {
        wtr: String,
    }

    impl TestVisitor {
        fn write_literal_char(&mut self, _c: char) -> fmt::Result {
            Ok(())
        }
        
        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            self.wtr.push(b as char);
            Ok(())
        }
        
        fn write_literal_class_byte(&mut self, _b: u8) -> fmt::Result {
            Ok(())
        }
    }

    enum HirKind {
        Literal(Literal),
    }

    enum Literal {
        Unicode(char),
        Byte(u8),
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut visitor = TestVisitor { wtr: String::new() };
    let hir = Hir { kind: HirKind::Literal(Literal::Byte(b'a')) };
    visitor.visit_pre(&hir)?;
    assert_eq!(visitor.wtr, "a");
    Ok(())
}

