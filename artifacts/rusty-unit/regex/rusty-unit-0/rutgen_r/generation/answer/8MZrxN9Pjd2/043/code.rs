// Answer 0

fn test_visit_pre_class_unicode() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
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

    let mut writer = TestWriter { output: String::new() };

    let hir = TestHir {
        kind: HirKind::Class(hir::Class::Unicode(vec![
            std::char::from_u32(0x0061).unwrap(),
            std::char::from_u32(0x007A).unwrap(),
        ])),
    };

    let result = visit_pre(&mut writer, &hir);
    assert!(result.is_ok());
    assert_eq!(writer.output, "[a-z]");
}

fn test_visit_pre_empty() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
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

    let mut writer = TestWriter { output: String::new() };

    let hir = TestHir { kind: HirKind::Empty };

    let result = visit_pre(&mut writer, &hir);
    assert!(result.is_ok());
    assert_eq!(writer.output, "");
}

fn test_visit_pre_class_bytes() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
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

    let mut writer = TestWriter { output: String::new() };

    let hir = TestHir {
        kind: HirKind::Class(hir::Class::Bytes(vec![
            0x61, // 'a'
            0x7A, // 'z'
        ])),
    };

    let result = visit_pre(&mut writer, &hir);
    assert!(result.is_ok());
    assert_eq!(writer.output, "(?-u:[a-z])");
}

fn test_visit_pre_class_unicode_with_err() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
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

    let mut writer = TestWriter { output: String::new() };

    let hir = TestHir {
        kind: HirKind::Class(hir::Class::Unicode(vec![
            std::char::from_u32(0x0061).unwrap(),
            std::char::from_u32(0x0062).unwrap(),
        ])),
    };

    let result = visit_pre(&mut writer, &hir);
    assert!(result.is_err());
}

#[test]
fn run_tests() {
    test_visit_pre_class_unicode();
    test_visit_pre_empty();
    test_visit_pre_class_bytes();
    test_visit_pre_class_unicode_with_err();
}

