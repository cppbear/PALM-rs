// Answer 0

fn test_visit_pre_unicode_literal() -> Result<(), fmt::Error> {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_literal_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }

        fn write_literal_byte(&mut self, _b: u8) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> TestVisitor<'a> {
        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            match *hir.kind() {
                HirKind::Literal(hir::Literal::Unicode(c)) => {
                    self.wtr.write_literal_char(c)?;
                }
                HirKind::Literal(hir::Literal::Byte(b)) => {
                    self.wtr.write_literal_byte(b)?;
                }
                _ => {}
            }
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    let mut visitor = TestVisitor { wtr: &mut writer };
    let unicode_hir = Hir::new(hir::Literal::Unicode('a'));
    let byte_hir = Hir::new(hir::Literal::Byte(0));

    assert_eq!(visitor.visit_pre(&unicode_hir), Ok(()));
    assert_eq!(visitor.visit_pre(&byte_hir).is_err(), true);

    Ok(())
}

fn test_visit_pre_empty() -> Result<(), fmt::Error> {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_literal_char(&mut self, _c: char) -> fmt::Result {
            Ok(())
        }

        fn write_literal_byte(&mut self, _b: u8) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> TestVisitor<'a> {
        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            match *hir.kind() {
                HirKind::Empty => {}
                HirKind::Literal(hir::Literal::Unicode(c)) => {
                    self.wtr.write_literal_char(c)?;
                }
                HirKind::Literal(hir::Literal::Byte(b)) => {
                    self.wtr.write_literal_byte(b)?;
                }
                _ => {}
            }
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    let mut visitor = TestVisitor { wtr: &mut writer };
    let empty_hir = Hir::new(hir::Kind::Empty);
    
    assert_eq!(visitor.visit_pre(&empty_hir), Ok(()));
    
    Ok(())
}

#[test]
fn run_tests() {
    test_visit_pre_unicode_literal().unwrap();
    test_visit_pre_empty().unwrap();
}

