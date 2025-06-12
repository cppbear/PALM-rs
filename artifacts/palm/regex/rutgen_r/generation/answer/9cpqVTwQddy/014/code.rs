// Answer 0

#[test]
fn test_fmt_class_ascii_lower() {
    use std::fmt::{self, Write};
    use regex_syntax::ast::{self, ClassAscii, ClassAsciiKind};

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };

    let ast = ClassAscii {
        kind: ClassAsciiKind::Lower,
        negated: false,
    };

    writer.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:lower:]");
}

#[test]
fn test_fmt_class_ascii_lower_negated() {
    use std::fmt::{self, Write};
    use regex_syntax::ast::{self, ClassAscii, ClassAsciiKind};

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };

    let ast = ClassAscii {
        kind: ClassAsciiKind::Lower,
        negated: true,
    };

    writer.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:^lower:]");
}

