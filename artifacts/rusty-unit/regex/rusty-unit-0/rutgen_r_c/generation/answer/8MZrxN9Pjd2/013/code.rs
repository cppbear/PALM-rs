// Answer 0

#[test]
fn test_visit_pre_with_unicode_negate() {
    use hir::{Hir, HirKind, Class, WordBoundary};
    use std::fmt::Write;

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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let hir = Hir {
        kind: HirKind::WordBoundary(WordBoundary::UnicodeNegate),
        info: HirInfo::default(),
    };

    let result = visitor.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, r"(?-u:\B)");
}

#[test]
fn test_visit_pre_with_unicode_class() {
    use hir::{Hir, HirKind, Class, ClassUnicode, ClassUnicodeRange, WordBoundary};
    use std::fmt::Write;

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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let class = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]); 
    let hir = Hir {
        kind: HirKind::Class(Class::Unicode(class)),
        info: HirInfo::default(),
    };

    let result = visitor.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "[a-z]");
}

#[test]
fn test_visit_pre_with_bytes_class() {
    use hir::{Hir, HirKind, Class, ClassBytes, ClassBytesRange};
    use std::fmt::Write;

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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let class = ClassBytes::new(vec![ClassBytesRange::new(0x30, 0x39)]); 
    let hir = Hir {
        kind: HirKind::Class(Class::Bytes(class)),
        info: HirInfo::default(),
    };

    let result = visitor.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "(?-u:[0-9])");
}

