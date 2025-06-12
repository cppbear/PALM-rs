// Answer 0

#[test]
fn test_fmt_literal_verbatim() {
    use std::fmt::Write;
    use regex_syntax::ast::{self, Literal, LiteralKind};

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

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let literal = Literal { c: 'a', kind: LiteralKind::Verbatim };
    
    writer.fmt_literal(&literal).unwrap();
    
    assert_eq!(writer.output, "a");
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_short() {
    use std::fmt::Write;
    use regex_syntax::ast::{self, Literal, LiteralKind, HexLiteralKind};

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

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let literal = Literal { c: 'b' as u32, kind: LiteralKind::HexFixed(HexLiteralKind::UnicodeShort) };
    
    writer.fmt_literal(&literal).unwrap();
    
    assert_eq!(writer.output, r"\u{00000062}");
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_long() {
    use std::fmt::Write;
    use regex_syntax::ast::{self, Literal, LiteralKind, HexLiteralKind};

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

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let literal = Literal { c: 'c' as u32, kind: LiteralKind::HexFixed(HexLiteralKind::UnicodeLong) };
    
    writer.fmt_literal(&literal).unwrap();
    
    assert_eq!(writer.output, r"\U{00000063}");
}

#[test]
fn test_fmt_literal_hex_fixed_x() {
    use std::fmt::Write;
    use regex_syntax::ast::{self, Literal, LiteralKind, HexLiteralKind};

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

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let literal = Literal { c: 'd' as u32, kind: LiteralKind::HexFixed(HexLiteralKind::X) };
    
    writer.fmt_literal(&literal).unwrap();
    
    assert_eq!(writer.output, r"\x{64}");
}

