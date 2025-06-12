// Answer 0

#[test]
fn test_fmt_literal_hex_brace_x() {
    use regex_syntax::ast::{self, LiteralKind, HexLiteralKind};
    use std::fmt::Write;

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
    let ast = ast::Literal { kind: LiteralKind::HexBrace(HexLiteralKind::X), c: 'A' };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\x{{41}}");
}

#[test]
fn test_fmt_literal_hex_brace_unicode_short() {
    use regex_syntax::ast::{self, LiteralKind, HexLiteralKind};
    use std::fmt::Write;

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
    let ast = ast::Literal { kind: LiteralKind::HexBrace(HexLiteralKind::UnicodeShort), c: 'B' };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\u{{0042}}");
}

#[test]
fn test_fmt_literal_hex_brace_unicode_long() {
    use regex_syntax::ast::{self, LiteralKind, HexLiteralKind};
    use std::fmt::Write;

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
    let ast = ast::Literal { kind: LiteralKind::HexBrace(HexLiteralKind::UnicodeLong), c: 'C' };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\U{{00000043}}");
}

#[test]
fn test_fmt_literal_octal() {
    use regex_syntax::ast::{self, LiteralKind};
    use std::fmt::Write;

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
    let ast = ast::Literal { kind: LiteralKind::Octal, c: '\x07' };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\7");
}

