// Answer 0

#[test]
fn test_fmt_class_ascii_xdigit_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    struct MockFormatter {
        wtr: MockWriter,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                wtr: MockWriter::new(),
            }
        }

        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> std::fmt::Result {
            // Implementation of fmt_class_ascii goes here,
            // For the sake of this test, you can copy the original function provided
            use ast::ClassAsciiKind::*;
            match ast.kind {
                Alnum if ast.negated => self.wtr.write_str("[:^alnum:]"),
                Alnum => self.wtr.write_str("[:alnum:]"),
                Alpha if ast.negated => self.wtr.write_str("[:^alpha:]"),
                Alpha => self.wtr.write_str("[:alpha:]"),
                Ascii if ast.negated => self.wtr.write_str("[:^ascii:]"),
                Ascii => self.wtr.write_str("[:ascii:]"),
                Blank if ast.negated => self.wtr.write_str("[:^blank:]"),
                Blank => self.wtr.write_str("[:blank:]"),
                Cntrl if ast.negated => self.wtr.write_str("[:^cntrl:]"),
                Cntrl => self.wtr.write_str("[:cntrl:]"),
                Digit if ast.negated => self.wtr.write_str("[:^digit:]"),
                Digit => self.wtr.write_str("[:digit:]"),
                Graph if ast.negated => self.wtr.write_str("[:^graph:]"),
                Graph => self.wtr.write_str("[:graph:]"),
                Lower if ast.negated => self.wtr.write_str("[:^lower:]"),
                Lower => self.wtr.write_str("[:lower:]"),
                Print if ast.negated => self.wtr.write_str("[:^print:]"),
                Print => self.wtr.write_str("[:print:]"),
                Punct if ast.negated => self.wtr.write_str("[:^punct:]"),
                Punct => self.wtr.write_str("[:punct:]"),
                Space if ast.negated => self.wtr.write_str("[:^space:]"),
                Space => self.wtr.write_str("[:space:]"),
                Upper if ast.negated => self.wtr.write_str("[:^upper:]"),
                Upper => self.wtr.write_str("[:upper:]"),
                Word if ast.negated => self.wtr.write_str("[:^word:]"),
                Word => self.wtr.write_str("[:word:]"),
                Xdigit if ast.negated => self.wtr.write_str("[:^xdigit:]"),
                Xdigit => self.wtr.write_str("[:xdigit:]"),
            }
        }
    }

    struct ClassAscii {
        kind: ast::ClassAsciiKind,
        negated: bool,
    }

    mod ast {
        pub enum ClassAsciiKind {
            Xdigit,
        }
    }

    let mut formatter = MockFormatter::new();
    let ast = ClassAscii {
        kind: ast::ClassAsciiKind::Xdigit,
        negated: true,
    };

    let result = formatter.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "[:^xdigit:]");
}

