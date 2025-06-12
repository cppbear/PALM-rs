// Answer 0

#[test]
fn test_fmt_class_ascii_digit() {
    use std::fmt;

    struct Writer {
        output: String,
    }

    impl fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct ClassAscii {
        kind: ClassAsciiKind,
        negated: bool,
    }

    enum ClassAsciiKind {
        Digit,
    }

    let mut writer = Writer { output: String::new() };
    let ast = ClassAscii { kind: ClassAsciiKind::Digit, negated: false };

    writer.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:digit:]");
}

#[test]
fn test_fmt_class_ascii_negated_digit() {
    use std::fmt;

    struct Writer {
        output: String,
    }

    impl fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct ClassAscii {
        kind: ClassAsciiKind,
        negated: bool,
    }

    enum ClassAsciiKind {
        Digit,
    }

    let mut writer = Writer { output: String::new() };
    let ast = ClassAscii { kind: ClassAsciiKind::Digit, negated: true };

    writer.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:^digit:]");
}

