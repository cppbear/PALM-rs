// Answer 0

#[test]
fn test_fmt_literal_verbatim() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }
    }

    use std::fmt::{self, Write};

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = ast::Literal { kind: ast::LiteralKind::Verbatim, c: 'a' };
    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, "a");
}

#[test]
fn test_fmt_literal_punctuation() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = ast::Literal { kind: ast::LiteralKind::Punctuation, c: '!' };
    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\!");
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_short() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = ast::Literal { kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeShort), c: 'A' };
    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\u{0041}");
}

#[test]
fn test_fmt_literal_special_tab() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = ast::Literal { kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab), c: '\t' };
    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\t");
}

