// Answer 0

#[test]
fn test_fmt_literal_verbatim() {
    use std::fmt::Write;
    use crate::ast::{self, LiteralKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
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
    let ast = ast::Literal {
        kind: LiteralKind::Verbatim,
        c: 'c',
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, "c");
}

#[test]
fn test_fmt_literal_hex_brace_x() {
    use std::fmt::Write;
    use crate::ast::{self, LiteralKind, HexLiteralKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
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
    let ast = ast::Literal {
        kind: LiteralKind::HexBrace(HexLiteralKind::X),
        c: 'A',
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\x{{41}}");
}

#[test]
fn test_fmt_literal_hex_brace_unicode() {
    use std::fmt::Write;
    use crate::ast::{self, LiteralKind, HexLiteralKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
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
    let ast = ast::Literal {
        kind: LiteralKind::HexBrace(HexLiteralKind::UnicodeLong),
        c: 'B',
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\U{{42}}");
}

