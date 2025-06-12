// Answer 0

#[test]
fn test_fmt_literal_hex_brace_x() {
    use std::fmt::Write;
    use regex_syntax::ast::{self, Literal, HexLiteralKind};

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

        fn write_char(&mut self, c: char) -> std::fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = Literal {
        c: 'A',
        kind: ast::LiteralKind::HexBrace(HexLiteralKind::X),
    };

    let result = fmt::write(&mut writer, |f| {
        f.fmt_literal(&ast)
    });

    assert!(result.is_ok());
    assert_eq!(writer.output, r"\x{{41}}");
}

#[test]
fn test_fmt_literal_punctuation() {
    use std::fmt::Write;
    use regex_syntax::ast::{self, Literal};

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

        fn write_char(&mut self, c: char) -> std::fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = Literal {
        c: '.',
        kind: ast::LiteralKind::Punctuation,
    };

    let result = fmt::write(&mut writer, |f| {
        f.fmt_literal(&ast)
    });

    assert!(result.is_ok());
    assert_eq!(writer.output, r"\.");
}

#[test]
fn test_fmt_literal_hex_brace_unicode_short() {
    use std::fmt::Write;
    use regex_syntax::ast::{self, Literal, HexLiteralKind};

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

        fn write_char(&mut self, c: char) -> std::fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let ast = Literal {
        c: 'B',
        kind: ast::LiteralKind::HexBrace(HexLiteralKind::UnicodeShort),
    };

    let result = fmt::write(&mut writer, |f| {
        f.fmt_literal(&ast)
    });

    assert!(result.is_ok());
    assert_eq!(writer.output, r"\u{{0042}}");
}

