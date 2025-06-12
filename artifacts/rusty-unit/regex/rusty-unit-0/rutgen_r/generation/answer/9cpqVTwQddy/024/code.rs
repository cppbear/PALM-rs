// Answer 0

#[test]
fn test_fmt_class_ascii_ascii() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestFormatter {
        wtr: Writer,
    }

    impl TestFormatter {
        fn new() -> Self {
            Self { wtr: Writer::new() }
        }

        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> std::fmt::Result {
            use ast::ClassAsciiKind::*;
            match ast.kind {
                ast::ClassAsciiKind::Ascii if ast.negated => self.wtr.write_str("[:^ascii:]"),
                ast::ClassAsciiKind::Ascii => self.wtr.write_str("[:ascii:]"),
                _ => Ok(()),
            }
        }
    }

    mod ast {
        pub struct ClassAscii {
            pub kind: ClassAsciiKind,
            pub negated: bool,
        }

        pub enum ClassAsciiKind {
            Alnum,
            Alpha,
            Ascii,
            Blank,
            Cntrl,
            Digit,
            Graph,
            Lower,
            Print,
            Punct,
            Space,
            Upper,
            Word,
            Xdigit,
        }
    }

    let mut formatter = TestFormatter::new();
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Ascii,
        negated: false,
    };

    let result = formatter.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "[:ascii:]");
}

