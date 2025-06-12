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

        fn write_literal_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    let mut writer = TestWriter { output: String::new() };
    
    let mut visitor = TestVisitor { wtr: &mut writer };

    let hir = Hir::from(HirKind::Literal(hir::Literal::Unicode('a')));
    visit_pre(&mut visitor, &hir).unwrap();

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

        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
            Ok(())
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    let mut writer = TestWriter { output: String::new() };
    
    let mut visitor = TestVisitor { wtr: &mut writer };

    let hir = Hir::from(HirKind::Literal(hir::Literal::Byte(b'a')));
    visit_pre(&mut visitor, &hir).unwrap();

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

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    let mut writer = TestWriter { output: String::new() };

    let classes = vec![char::from(97)..=char::from(97)]; // class 'a'
    let hir = Hir::from(HirKind::Class(hir::Class::Unicode(classes)));

    let mut visitor = TestVisitor { wtr: &mut writer };
    visit_pre(&mut visitor, &hir).unwrap();

    assert_eq!(writer.output, "[a]");
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

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    let mut writer = TestWriter { output: String::new() };

    let classes = vec![b'a'..=b'a']; // class 'a'
    let hir = Hir::from(HirKind::Class(hir::Class::Bytes(classes)));

    let mut visitor = TestVisitor { wtr: &mut writer };
    visit_pre(&mut visitor, &hir).unwrap();

    assert_eq!(writer.output, "(?-u:[a])");
}

