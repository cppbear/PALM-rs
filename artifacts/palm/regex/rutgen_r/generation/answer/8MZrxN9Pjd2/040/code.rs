// Answer 0

fn test_visit_pre_empty() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl TestVisitor<'_> {
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

    let mut writer = TestWriter { output: String::new() };
    let mut visitor = TestVisitor { wtr: &mut writer };
    let hir = Hir::new(HirKind::Empty);

    let result = visitor.visit_pre(&hir);
    assert_eq!(result, Ok(()));
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
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl TestVisitor<'_> {
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

    let mut writer = TestWriter { output: String::new() };
    let mut visitor = TestVisitor { wtr: &mut writer };
    let ranges = vec![UnicodeRange::new('a', 'a')]; // Range where start equals end
    let cls = Class::Unicode(ranges);
    let hir = Hir::new(HirKind::Class(cls));

    let result = visitor.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "[a]");
}

fn test_visit_pre_bytes_class() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl TestVisitor<'_> {
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

    let mut writer = TestWriter { output: String::new() };
    let mut visitor = TestVisitor { wtr: &mut writer };
    let ranges = vec![ByteRange::new(0, 0)]; // Range where start equals end
    let cls = Class::Bytes(ranges);
    let hir = Hir::new(HirKind::Class(cls));

    let result = visitor.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "(?-u:[0-0])");
}

