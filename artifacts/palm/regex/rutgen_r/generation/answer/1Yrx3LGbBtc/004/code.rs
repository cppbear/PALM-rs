// Answer 0

#[test]
fn test_fmt_class_perl_space() {
    use regex_syntax::ast::{ClassPerl, ClassPerlKind};
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = ClassPerl {
        kind: ClassPerlKind::Space,
        negated: false,
    };

    let result = writer.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\s");
}

#[test]
fn test_fmt_class_perl_space_negated() {
    use regex_syntax::ast::{ClassPerl, ClassPerlKind};
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = ClassPerl {
        kind: ClassPerlKind::Space,
        negated: true,
    };

    let result = writer.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\S");
}

