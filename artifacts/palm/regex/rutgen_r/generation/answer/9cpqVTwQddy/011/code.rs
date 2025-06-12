// Answer 0

#[test]
fn test_fmt_class_ascii_negated_print() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct FmtClassAscii<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> FmtClassAscii<'a> {
        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {
            // Original function implementation should be here.
            unimplemented!()
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

        #[derive(Debug)]
        pub enum ClassAsciiKind {
            Print,
            // Other variants omitted for brevity
        }
    }

    let mut writer = MockWriter::new();
    let mut formatter = FmtClassAscii { wtr: &mut writer };
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Print,
        negated: true,
    };
    
    let result = formatter.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:^print:]");
}

#[test]
fn test_fmt_class_ascii_alnum_negated() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct FmtClassAscii<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> FmtClassAscii<'a> {
        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {
            // Original function implementation should be here.
            unimplemented!()
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

        #[derive(Debug)]
        pub enum ClassAsciiKind {
            Alnum,
            // Other variants omitted for brevity
        }
    }

    let mut writer = MockWriter::new();
    let mut formatter = FmtClassAscii { wtr: &mut writer };
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };
    
    let result = formatter.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:^alnum:]");
}

