// Answer 0

#[test]
fn test_fmt_class_unicode_one_letter() {
    use std::fmt::Write;
    use regex_syntax::ast::{ClassUnicode, ClassUnicodeKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = ClassUnicode {
        negated: false,
        kind: ClassUnicodeKind::OneLetter('a'),
    };
    writer.fmt_class_unicode(&ast).unwrap();
    assert_eq!(writer.output, r"\p" + "a");
}

#[test]
fn test_fmt_class_unicode_named() {
    use std::fmt::Write;
    use regex_syntax::ast::{ClassUnicode, ClassUnicodeKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = ClassUnicode {
        negated: false,
        kind: ClassUnicodeKind::Named("L".to_string()),
    };
    writer.fmt_class_unicode(&ast).unwrap();
    assert_eq!(writer.output, r"\p" + "{L}");
}

#[test]
fn test_fmt_class_unicode_named_value_equal() {
    use std::fmt::Write;
    use regex_syntax::ast::{ClassUnicode, ClassUnicodeKind, ClassUnicodeOpKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = ClassUnicode {
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::Equal,
            name: "Alpha".to_string(),
            value: "A".to_string(),
        },
    };
    writer.fmt_class_unicode(&ast).unwrap();
    assert_eq!(writer.output, r"\p" + "{Alpha=A}");
}

#[test]
fn test_fmt_class_unicode_negated() {
    use std::fmt::Write;
    use regex_syntax::ast::{ClassUnicode, ClassUnicodeKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = ClassUnicode {
        negated: true,
        kind: ClassUnicodeKind::OneLetter('b'),
    };
    writer.fmt_class_unicode(&ast).unwrap();
    assert_eq!(writer.output, r"\P" + "b");
}

#[test]
fn test_fmt_class_unicode_named_value_not_equal() {
    use std::fmt::Write;
    use regex_syntax::ast::{ClassUnicode, ClassUnicodeKind, ClassUnicodeOpKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = ClassUnicode {
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: "Beta".to_string(),
            value: "B".to_string(),
        },
    };
    writer.fmt_class_unicode(&ast).unwrap();
    assert_eq!(writer.output, r"\p" + "{Beta!=B}");
}

