// Answer 0

#[test]
fn test_visit_pre_bytes_class() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let class_bytes = ClassBytes::new(vec![ClassBytesRange::new(65, 70)]);
    let hir = Hir::class(hir::Class::Bytes(class_bytes));

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_unicode_class() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let class_unicode = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]);
    let hir = Hir::class(hir::Class::Unicode(class_unicode));

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_unicode_literal() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let hir = Hir::literal(hir::Literal::Unicode('Z'));

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_byte_literal() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let hir = Hir::literal(hir::Literal::Byte(65));

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

#[test]
#[should_panic]
fn test_visit_pre_bytes_class_error() {
    struct MockFailingWriter;

    impl fmt::Write for MockFailingWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut writer = MockFailingWriter;
    let class_bytes = ClassBytes::new(vec![ClassBytesRange::new(65, 70)]);
    let hir = Hir::class(hir::Class::Bytes(class_bytes));

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let _ = visitor.visit_pre(&hir);
}

