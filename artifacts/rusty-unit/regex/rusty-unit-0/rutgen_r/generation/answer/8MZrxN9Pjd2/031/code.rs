// Answer 0

fn test_visit_pre_unicode_literal() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
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

    let mut writer = TestWriter { output: String::new() };
    let hir = TestHir {
        kind: HirKind::Literal(hir::Literal::Unicode('a')),
    };
    
    assert!(visit_pre(&mut writer, &hir).is_ok());
    assert_eq!(writer.output, "a");
}

fn test_visit_pre_byte_literal() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
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

    let mut writer = TestWriter { output: String::new() };
    let hir = TestHir {
        kind: HirKind::Literal(hir::Literal::Byte(b'a')),
    };
    
    assert!(visit_pre(&mut writer, &hir).is_ok());
    assert_eq!(writer.output, "a");
}

fn test_visit_pre_unicode_class() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
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
    
    let mut writer = TestWriter { output: String::new() };
    let ranges = vec![char::from('a'), char::from('b')];
    let hir_class = hir::Class::Unicode(ranges.iter().map(|&c| hir::ClassRange::new(c, c)).collect::<Vec<_>>());
    let hir = TestHir {
        kind: HirKind::Class(hir_class),
    };
    
    assert!(visit_pre(&mut writer, &hir).is_ok());
    assert_eq!(writer.output, "[a-b]");
}

fn test_visit_pre_byte_class() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_literal_class_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
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
    let ranges = vec![b'a', b'b'];
    let hir_class = hir::Class::Bytes(ranges.iter().map(|&b| hir::ClassRange::new(b, b)).collect::<Vec<_>>());
    let hir = TestHir {
        kind: HirKind::Class(hir_class),
    };

    assert!(visit_pre(&mut writer, &hir).is_ok());
    assert_eq!(writer.output, "(?-u:[a-b])");
}

