// Answer 0

#[test]
fn test_fmt_class_ascii_space_negated() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
        
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct FmtClassAscii {
        wtr: MockWriter,
    }

    impl FmtClassAscii {
        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> std::fmt::Result {
            use ast::ClassAsciiKind::*;
            match ast.kind {
                ast::ClassAsciiKind::Alnum if ast.negated => self.wtr.write_str("[:^alnum:]"),
                ast::ClassAsciiKind::Alnum => self.wtr.write_str("[:alnum:]"),
                ast::ClassAsciiKind::Alpha if ast.negated => self.wtr.write_str("[:^alpha:]"),
                ast::ClassAsciiKind::Alpha => self.wtr.write_str("[:alpha:]"),
                ast::ClassAsciiKind::Ascii if ast.negated => self.wtr.write_str("[:^ascii:]"),
                ast::ClassAsciiKind::Ascii => self.wtr.write_str("[:ascii:]"),
                ast::ClassAsciiKind::Blank if ast.negated => self.wtr.write_str("[:^blank:]"),
                ast::ClassAsciiKind::Blank => self.wtr.write_str("[:blank:]"),
                ast::ClassAsciiKind::Cntrl if ast.negated => self.wtr.write_str("[:^cntrl:]"),
                ast::ClassAsciiKind::Cntrl => self.wtr.write_str("[:cntrl:]"),
                ast::ClassAsciiKind::Digit if ast.negated => self.wtr.write_str("[:^digit:]"),
                ast::ClassAsciiKind::Digit => self.wtr.write_str("[:digit:]"),
                ast::ClassAsciiKind::Graph if ast.negated => self.wtr.write_str("[:^graph:]"),
                ast::ClassAsciiKind::Graph => self.wtr.write_str("[:graph:]"),
                ast::ClassAsciiKind::Lower if ast.negated => self.wtr.write_str("[:^lower:]"),
                ast::ClassAsciiKind::Lower => self.wtr.write_str("[:lower:]"),
                ast::ClassAsciiKind::Print if ast.negated => self.wtr.write_str("[:^print:]"),
                ast::ClassAsciiKind::Print => self.wtr.write_str("[:print:]"),
                ast::ClassAsciiKind::Punct if ast.negated => self.wtr.write_str("[:^punct:]"),
                ast::ClassAsciiKind::Punct => self.wtr.write_str("[:punct:]"),
                ast::ClassAsciiKind::Space if ast.negated => self.wtr.write_str("[:^space:]"),
                ast::ClassAsciiKind::Space => self.wtr.write_str("[:space:]"),
                ast::ClassAsciiKind::Upper if ast.negated => self.wtr.write_str("[:^upper:]"),
                ast::ClassAsciiKind::Upper => self.wtr.write_str("[:upper:]"),
                ast::ClassAsciiKind::Word if ast.negated => self.wtr.write_str("[:^word:]"),
                ast::ClassAsciiKind::Word => self.wtr.write_str("[:word:]"),
                ast::ClassAsciiKind::Xdigit if ast.negated => self.wtr.write_str("[:^xdigit:]"),
                ast::ClassAsciiKind::Xdigit => self.wtr.write_str("[:xdigit:]"),
            }
        }
    }

    struct ClassAscii {
        kind: ast::ClassAsciiKind,
        negated: bool,
    }

    let mut writer = MockWriter::new();
    let mut fmt_class_ascii = FmtClassAscii { wtr: writer };

    let ast = ClassAscii {
        kind: ast::ClassAsciiKind::Space,
        negated: true,
    };

    fmt_class_ascii.fmt_class_ascii(&ast).unwrap();
    
    assert_eq!(fmt_class_ascii.wtr.output, "[:^space:]");
}

