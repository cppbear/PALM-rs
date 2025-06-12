// Answer 0

#[test]
fn test_fmt_class_ascii_alnum_negated() {
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

    struct TestFormatter {
        wtr: TestWriter,
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter {
                wtr: TestWriter::new(),
            }
        }

        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {
            use ast::ClassAsciiKind::*;
            match ast.kind {
                Alnum if ast.negated => self.wtr.output.write_str("[:^alnum:]"),
                Alnum => self.wtr.output.write_str("[:alnum:]"),
                Alpha if ast.negated => self.wtr.output.write_str("[:^alpha:]"),
                Alpha => self.wtr.output.write_str("[:alpha:]"),
                Ascii if ast.negated => self.wtr.output.write_str("[:^ascii:]"),
                Ascii => self.wtr.output.write_str("[:ascii:]"),
                Blank if ast.negated => self.wtr.output.write_str("[:^blank:]"),
                Blank => self.wtr.output.write_str("[:blank:]"),
                Cntrl if ast.negated => self.wtr.output.write_str("[:^cntrl:]"),
                Cntrl => self.wtr.output.write_str("[:cntrl:]"),
                Digit if ast.negated => self.wtr.output.write_str("[:^digit:]"),
                Digit => self.wtr.output.write_str("[:digit:]"),
                Graph if ast.negated => self.wtr.output.write_str("[:^graph:]"),
                Graph => self.wtr.output.write_str("[:graph:]"),
                Lower if ast.negated => self.wtr.output.write_str("[:^lower:]"),
                Lower => self.wtr.output.write_str("[:lower:]"),
                Print if ast.negated => self.wtr.output.write_str("[:^print:]"),
                Print => self.wtr.output.write_str("[:print:]"),
                Punct if ast.negated => self.wtr.output.write_str("[:^punct:]"),
                Punct => self.wtr.output.write_str("[:punct:]"),
                Space if ast.negated => self.wtr.output.write_str("[:^space:]"),
                Space => self.wtr.output.write_str("[:space:]"),
                Upper if ast.negated => self.wtr.output.write_str("[:^upper:]"),
                Upper => self.wtr.output.write_str("[:upper:]"),
                Word if ast.negated => self.wtr.output.write_str("[:^word:]"),
                Word => self.wtr.output.write_str("[:word:]"),
                Xdigit if ast.negated => self.wtr.output.write_str("[:^xdigit:]"),
                Xdigit => self.wtr.output.write_str("[:xdigit:]"),
            }
            Ok(())
        }
    }
    
    struct ClassAscii {
        kind: ast::ClassAsciiKind,
        negated: bool,
    }

    mod ast {
        #[derive(Clone, Copy)]
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
        
        pub struct ClassAscii {
            pub kind: ClassAsciiKind,
            pub negated: bool,
        }
    }

    let mut formatter = TestFormatter::new();
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };
    formatter.fmt_class_ascii(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "[:^alpha:]");
}

#[test]
fn test_fmt_class_ascii_alpha_non_negated() {
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

    struct TestFormatter {
        wtr: TestWriter,
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter {
                wtr: TestWriter::new(),
            }
        }

        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {
            use ast::ClassAsciiKind::*;
            match ast.kind {
                Alnum if ast.negated => self.wtr.output.write_str("[:^alnum:]"),
                Alnum => self.wtr.output.write_str("[:alnum:]"),
                Alpha if ast.negated => self.wtr.output.write_str("[:^alpha:]"),
                Alpha => self.wtr.output.write_str("[:alpha:]"),
                Ascii if ast.negated => self.wtr.output.write_str("[:^ascii:]"),
                Ascii => self.wtr.output.write_str("[:ascii:]"),
                Blank if ast.negated => self.wtr.output.write_str("[:^blank:]"),
                Blank => self.wtr.output.write_str("[:blank:]"),
                Cntrl if ast.negated => self.wtr.output.write_str("[:^cntrl:]"),
                Cntrl => self.wtr.output.write_str("[:cntrl:]"),
                Digit if ast.negated => self.wtr.output.write_str("[:^digit:]"),
                Digit => self.wtr.output.write_str("[:digit:]"),
                Graph if ast.negated => self.wtr.output.write_str("[:^graph:]"),
                Graph => self.wtr.output.write_str("[:graph:]"),
                Lower if ast.negated => self.wtr.output.write_str("[:^lower:]"),
                Lower => self.wtr.output.write_str("[:lower:]"),
                Print if ast.negated => self.wtr.output.write_str("[:^print:]"),
                Print => self.wtr.output.write_str("[:print:]"),
                Punct if ast.negated => self.wtr.output.write_str("[:^punct:]"),
                Punct => self.wtr.output.write_str("[:punct:]"),
                Space if ast.negated => self.wtr.output.write_str("[:^space:]"),
                Space => self.wtr.output.write_str("[:space:]"),
                Upper if ast.negated => self.wtr.output.write_str("[:^upper:]"),
                Upper => self.wtr.output.write_str("[:upper:]"),
                Word if ast.negated => self.wtr.output.write_str("[:^word:]"),
                Word => self.wtr.output.write_str("[:word:]"),
                Xdigit if ast.negated => self.wtr.output.write_str("[:^xdigit:]"),
                Xdigit => self.wtr.output.write_str("[:xdigit:]"),
            }
            Ok(())
        }
    }
    
    struct ClassAscii {
        kind: ast::ClassAsciiKind,
        negated: bool,
    }

    mod ast {
        #[derive(Clone, Copy)]
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
        
        pub struct ClassAscii {
            pub kind: ClassAsciiKind,
            pub negated: bool,
        }
    }

    let mut formatter = TestFormatter::new();
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };
    formatter.fmt_class_ascii(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "[:alpha:]");
}

#[test]
fn test_fmt_class_ascii_ascii_negated() {
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

    struct TestFormatter {
        wtr: TestWriter,
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter {
                wtr: TestWriter::new(),
            }
        }

        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {
            use ast::ClassAsciiKind::*;
            match ast.kind {
                Alnum if ast.negated => self.wtr.output.write_str("[:^alnum:]"),
                Alnum => self.wtr.output.write_str("[:alnum:]"),
                Alpha if ast.negated => self.wtr.output.write_str("[:^alpha:]"),
                Alpha => self.wtr.output.write_str("[:alpha:]"),
                Ascii if ast.negated => self.wtr.output.write_str("[:^ascii:]"),
                Ascii => self.wtr.output.write_str("[:ascii:]"),
                Blank if ast.negated => self.wtr.output.write_str("[:^blank:]"),
                Blank => self.wtr.output.write_str("[:blank:]"),
                Cntrl if ast.negated => self.wtr.output.write_str("[:^cntrl:]"),
                Cntrl => self.wtr.output.write_str("[:cntrl:]"),
                Digit if ast.negated => self.wtr.output.write_str("[:^digit:]"),
                Digit => self.wtr.output.write_str("[:digit:]"),
                Graph if ast.negated => self.wtr.output.write_str("[:^graph:]"),
                Graph => self.wtr.output.write_str("[:graph:]"),
                Lower if ast.negated => self.wtr.output.write_str("[:^lower:]"),
                Lower => self.wtr.output.write_str("[:lower:]"),
                Print if ast.negated => self.wtr.output.write_str("[:^print:]"),
                Print => self.wtr.output.write_str("[:print:]"),
                Punct if ast.negated => self.wtr.output.write_str("[:^punct:]"),
                Punct => self.wtr.output.write_str("[:punct:]"),
                Space if ast.negated => self.wtr.output.write_str("[:^space:]"),
                Space => self.wtr.output.write_str("[:space:]"),
                Upper if ast.negated => self.wtr.output.write_str("[:^upper:]"),
                Upper => self.wtr.output.write_str("[:upper:]"),
                Word if ast.negated => self.wtr.output.write_str("[:^word:]"),
                Word => self.wtr.output.write_str("[:word:]"),
                Xdigit if ast.negated => self.wtr.output.write_str("[:^xdigit:]"),
                Xdigit => self.wtr.output.write_str("[:xdigit:]"),
            }
            Ok(())
        }
    }
    
    struct ClassAscii {
        kind: ast::ClassAsciiKind,
        negated: bool,
    }

    mod ast {
        #[derive(Clone, Copy)]
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
        
        pub struct ClassAscii {
            pub kind: ClassAsciiKind,
            pub negated: bool,
        }
    }

    let mut formatter = TestFormatter::new();
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Ascii,
        negated: true,
    };
    formatter.fmt_class_ascii(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "[:^ascii:]");
}

