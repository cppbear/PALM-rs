// Answer 0

#[test]
fn test_fmt_literal_special_characters() {
    use std::fmt::Write;
    use regex_syntax::ast::{self, Literal, SpecialLiteralKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    
    let ast_bell = Literal { kind: ast::LiteralKind::Special(SpecialLiteralKind::Bell), c: '\x07' };
    let ast_tab = Literal { kind: ast::LiteralKind::Special(SpecialLiteralKind::Tab), c: '\t' };
    let ast_carriage_return = Literal { kind: ast::LiteralKind::Special(SpecialLiteralKind::CarriageReturn), c: '\r' };
    let ast_line_feed = Literal { kind: ast::LiteralKind::Special(SpecialLiteralKind::LineFeed), c: '\n' };

    writer.fmt_literal(&ast_bell).unwrap();
    assert_eq!(writer.output, r"\a");

    writer.output.clear();
    writer.fmt_literal(&ast_tab).unwrap();
    assert_eq!(writer.output, r"\t");

    writer.output.clear();
    writer.fmt_literal(&ast_carriage_return).unwrap();
    assert_eq!(writer.output, r"\r");

    writer.output.clear();
    writer.fmt_literal(&ast_line_feed).unwrap();
    assert_eq!(writer.output, r"\n");
}

#[test]
fn test_fmt_literal_hex_fixed_characters() {
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

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    
    let ast_hex_fixed_x = Literal { kind: ast::LiteralKind::HexFixed(HexLiteralKind::X), c: 'A' };
    let ast_hex_fixed_unicode_short = Literal { kind: ast::LiteralKind::HexFixed(HexLiteralKind::UnicodeShort), c: 'ß' };
    let ast_hex_fixed_unicode_long = Literal { kind: ast::LiteralKind::HexFixed(HexLiteralKind::UnicodeLong), c: '𝄞' };

    writer.fmt_literal(&ast_hex_fixed_x).unwrap();
    assert_eq!(writer.output, r"\x41");

    writer.output.clear();
    writer.fmt_literal(&ast_hex_fixed_unicode_short).unwrap();
    assert_eq!(writer.output, r"\u00DF");

    writer.output.clear();
    writer.fmt_literal(&ast_hex_fixed_unicode_long).unwrap();
    assert_eq!(writer.output, r"\U0001D11E");
}

