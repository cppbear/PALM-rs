// Answer 0

#[test]
fn test_fmt_literal_hex_fixed_unicode_short() {
    use std::fmt::Write;
    use regex_syntax::ast::{Literal, HexLiteralKind};
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
    let ast = Literal {
        c: 'A' as u32,
        kind: HexFixed(HexLiteralKind::UnicodeShort),
    };
    
    let result = writer.fmt_literal(&ast);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\u0041");
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_long() {
    use std::fmt::Write;
    use regex_syntax::ast::{Literal, HexLiteralKind};
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
    let ast = Literal {
        c: 'A' as u32,
        kind: HexFixed(HexLiteralKind::UnicodeLong),
    };
    
    let result = writer.fmt_literal(&ast);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\U00000041");
}

#[test]
fn test_fmt_literal_octal() {
    use std::fmt::Write;
    use regex_syntax::ast::{Literal, LiteralKind};
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
    let ast = Literal {
        c: '7' as u32,
        kind: Octal,
    };
    
    let result = writer.fmt_literal(&ast);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\7");
}

#[test]
fn test_fmt_literal_hex_fixed_x() {
    use std::fmt::Write;
    use regex_syntax::ast::{Literal, HexLiteralKind};
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
    let ast = Literal {
        c: 'A' as u32,
        kind: HexFixed(HexLiteralKind::X),
    };
    
    let result = writer.fmt_literal(&ast);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\x41");
}

