// Answer 0

#[test]
fn test_fmt_class_ascii_blank_negated() {
    use ast::{ClassAscii, ClassAsciiKind};

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
    let class_ascii = ClassAscii {
        span: Span {}, // Assume Span is defined
        kind: ClassAsciiKind::Blank,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_class_ascii(&class_ascii).unwrap();

    assert_eq!(writer.output, "[:^blank:]");
}

#[test]
fn test_fmt_class_ascii_blank_non_negated() {
    use ast::{ClassAscii, ClassAsciiKind};

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
    let class_ascii = ClassAscii {
        span: Span {}, // Assume Span is defined
        kind: ClassAsciiKind::Blank,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_class_ascii(&class_ascii).unwrap();

    assert_eq!(writer.output, "[:blank:]");
} 

#[test]
fn test_fmt_class_ascii_alnum_negated() {
    use ast::{ClassAscii, ClassAsciiKind};

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
    let class_ascii = ClassAscii {
        span: Span {}, // Assume Span is defined
        kind: ClassAsciiKind::Alnum,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_class_ascii(&class_ascii).unwrap();

    assert_eq!(writer.output, "[:^alnum:]");
}

#[test]
fn test_fmt_class_ascii_alpha_non_negated() {
    use ast::{ClassAscii, ClassAsciiKind};

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
    let class_ascii = ClassAscii {
        span: Span {}, // Assume Span is defined
        kind: ClassAsciiKind::Alpha,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_class_ascii(&class_ascii).unwrap();
    
    assert_eq!(writer.output, "[:alpha:]");
} 

#[test]
fn test_fmt_class_ascii_digit_negated() {
    use ast::{ClassAscii, ClassAsciiKind};

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
    let class_ascii = ClassAscii {
        span: Span {}, // Assume Span is defined
        kind: ClassAsciiKind::Digit,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_class_ascii(&class_ascii).unwrap();

    assert_eq!(writer.output, "[:^digit:]");
} 

#[test]
fn test_fmt_class_ascii_space_non_negated() {
    use ast::{ClassAscii, ClassAsciiKind};

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
    let class_ascii = ClassAscii {
        span: Span {}, // Assume Span is defined
        kind: ClassAsciiKind::Space,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_class_ascii(&class_ascii).unwrap();
    
    assert_eq!(writer.output, "[:space:]");
}

