// Answer 0

fn fmt_literal_test() {
    use std::fmt::{self, Write};
    
    struct Writer {
        output: String,
    }
    
    impl Writer {
        fn new() -> Self {
            Self {
                output: String::new(),
            }
        }
    }
    
    impl fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }
    
    struct Literal {
        kind: ast::LiteralKind,
        c: char,
    }
    
    mod ast {
        pub enum LiteralKind {
            Special(SpecialLiteralKind),
            Octal,
            // other kinds...
        }
        
        pub enum SpecialLiteralKind {
            Bell,
            FormFeed,
            Tab,
            LineFeed,
            CarriageReturn,
            VerticalTab,
            Space,
        }
    }
    
    // Test for Special Literals
    {
        let mut writer = Writer::new();
        let ast = Literal {
            kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
            c: '\t',
        };
        let result = fmt_literal(&mut writer, &ast);
        assert!(result.is_ok());
        assert_eq!(writer.output, r"\t");
    }
    
    {
        let mut writer = Writer::new();
        let ast = Literal {
            kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
            c: '\r',
        };
        let result = fmt_literal(&mut writer, &ast);
        assert!(result.is_ok());
        assert_eq!(writer.output, r"\r");
    }
    
    // Test for Octal
    {
        let mut writer = Writer::new();
        let ast = Literal {
            kind: ast::LiteralKind::Octal,
            c: 'A',
        };
        let result = fmt_literal(&mut writer, &ast);
        assert!(result.is_ok());
        assert_eq!(writer.output, r"\101"); // 65 in octal
    }
    
    // Test for other Special Literals
    {
        let mut writer = Writer::new();
        let ast = Literal {
            kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
            c: '\u{07}',
        };
        let result = fmt_literal(&mut writer, &ast);
        assert!(result.is_ok());
        assert_eq!(writer.output, r"\a");
    }
    
    {
        let mut writer = Writer::new();
        let ast = Literal {
            kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
            c: '\n',
        };
        let result = fmt_literal(&mut writer, &ast);
        assert!(result.is_ok());
        assert_eq!(writer.output, r"\n");
    }
}

