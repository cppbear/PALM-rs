// Answer 0

fn test_fmt_literal_special_carriage_return() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_char(&mut self, c: char) -> std::fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = Writer::new();
    
    struct AstLiteral {
        kind: ast::LiteralKind,
        c: char,
    }
    
    let ast = AstLiteral {
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\r");
}

fn test_fmt_literal_special_vertical_tab() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_char(&mut self, c: char) -> std::fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = Writer::new();
    
    struct AstLiteral {
        kind: ast::LiteralKind,
        c: char,
    }
    
    let ast = AstLiteral {
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\x0B',
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\v");
}

fn test_fmt_literal_special_bell() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_char(&mut self, c: char) -> std::fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = Writer::new();
    
    struct AstLiteral {
        kind: ast::LiteralKind,
        c: char,
    }
    
    let ast = AstLiteral {
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\x07',
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\a");
}

fn test_fmt_literal_hex_brace_x() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_char(&mut self, c: char) -> std::fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = Writer::new();
    
    struct AstLiteral {
        kind: ast::LiteralKind,
        c: char,
    }
    
    let ast = AstLiteral {
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::X),
        c: 'A',
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\x{{41}}");
}

