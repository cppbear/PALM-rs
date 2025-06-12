// Answer 0

#[test]
fn test_fmt_class_ascii_lower_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    struct Formatter {
        wtr: MockWriter,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter { wtr: MockWriter::new() }
        }
    
        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {
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
        pub struct ClassAscii {
            pub kind: ClassAsciiKind,
            pub negated: bool,
        }
        
        #[derive(Copy, Clone)]
        pub enum ClassAsciiKind {
            Lower,
            // other variants...
        }
    }

    let mut formatter = Formatter::new();
    let class_ascii = ast::ClassAscii { kind: ast::ClassAsciiKind::Lower, negated: true };
    let result = formatter.fmt_class_ascii(&class_ascii);
    
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "[:^lower:]");
}

