// Answer 0

#[test]
fn test_fmt_class_ascii_alnum_negated() {
    use std::fmt;

    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer {
                output: String::new(),
            }
        }
    }

    impl fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestStruct {
        wtr: Writer,
    }

    impl TestStruct {
        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {
            use ast::ClassAsciiKind::*;
            match ast.kind {
                Alnum if ast.negated => self.wtr.write_str("[:^alnum:]"),
                Alnum => self.wtr.write_str("[:alnum:]"),
                // Other variants omitted for brevity...
                _ => Ok(())
            }
        }
    }

    mod ast {
        pub enum ClassAsciiKind {
            Alnum,
            // Other variants omitted for brevity...
        }

        pub struct ClassAscii {
            pub kind: ClassAsciiKind,
            pub negated: bool,
        }
    }

    let mut writer = Writer::new();
    let mut test_struct = TestStruct { wtr: writer };

    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };

    test_struct.fmt_class_ascii(&ast).unwrap();
    assert_eq!(test_struct.wtr.output, "[:^alnum:]");
}

