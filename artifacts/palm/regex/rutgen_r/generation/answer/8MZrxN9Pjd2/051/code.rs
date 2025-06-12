// Answer 0

#[test]
fn test_visit_pre_empty() {
    struct DummyWriter {
        output: String,
    }

    impl DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut writer = DummyWriter { output: String::new() };
    let hir = TestHir { kind: HirKind::Empty };
    assert_eq!(writer.visit_pre(&hir), Ok(()));
}

#[test]
fn test_visit_pre_literal_unicode() {
    struct DummyWriter {
        output: String,
    }

    impl DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_literal_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut writer = DummyWriter { output: String::new() };
    let hir = TestHir { kind: HirKind::Literal(hir::Literal::Unicode('a')) };
    assert_eq!(writer.visit_pre(&hir), Ok(()));
    assert_eq!(writer.output, "a");
}

#[test]
fn test_visit_pre_literal_byte() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl DummyWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }

        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b);
            Ok(())
        }
    }

    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut writer = DummyWriter { output: vec![] };
    let hir = TestHir { kind: HirKind::Literal(hir::Literal::Byte(b'a')) };
    assert_eq!(writer.visit_pre(&hir), Ok(()));
    assert_eq!(writer.output, vec![b'a']);
}

