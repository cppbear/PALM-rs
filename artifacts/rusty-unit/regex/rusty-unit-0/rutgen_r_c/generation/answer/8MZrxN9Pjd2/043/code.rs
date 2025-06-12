// Answer 0

fn test_visit_pre_class_unicode() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let cls = {
        let mut class = ClassUnicode::new(vec![
            ClassUnicodeRange::new('a', 'b'),
            ClassUnicodeRange::new('d', 'e'),
        ]);
        class.push(ClassUnicodeRange::new('c', 'c')); // Add equal range, should be ignored
        class
    };

    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(cls)),
        info: HirInfo::default(),
    };

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "[a-b][d-e]");
}

fn test_visit_pre_class_bytes() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let cls = {
        let mut class = ClassBytes::new(vec![
            ClassBytesRange::new(1, 2),
            ClassBytesRange::new(4, 5),
        ]);
        class.push(ClassBytesRange::new(3, 3)); // Add equal range, should be ignored
        class
    };

    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(cls)),
        info: HirInfo::default(),
    };

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "(?-u:[1-2][4-5])");
}

fn test_visit_pre_empty() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let hir = Hir {
        kind: HirKind::Empty,
        info: HirInfo::default(),
    };

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert!(visitor.wtr.output.is_empty());
}

fn test_visit_pre_panic_condition() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let cls = {
        let class = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]);
        // Only one range for the panic case
        class
    };

    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(cls)),
        info: HirInfo::default(),
    };

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert!(visitor.wtr.output.contains("-")); // Should have a hyphen
    assert!(visitor.wtr.output.contains("a"));
    assert!(visitor.wtr.output.contains("z"));
}

