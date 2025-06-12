// Answer 0

#[test]
fn test_fmt_class_ascii_word_negated() {
    use std::io::Write;
    use std::fmt;

    struct MockWriter {
        output: String,
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> fmt::Result {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(())
        }

        fn flush(&mut self) -> fmt::Result {
            Ok(())
        }
    }

    struct MockFormatter {
        wtr: MockWriter,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                wtr: MockWriter { output: String::new() },
            }
        }

        fn fmt_class_ascii(&mut self, ast: &ClassAscii) -> fmt::Result {
            use ast::ClassAsciiKind::*;
            match ast.kind {
                ClassAsciiKind::Word if ast.negated => self.wtr.write_str("[:^word:]"),
                ClassAsciiKind::Word => self.wtr.write_str("[:word:]"),
                // other match arms omitted for brevity
            }
        }
    }

    mod ast {
        pub struct ClassAscii {
            pub kind: ClassAsciiKind,
            pub negated: bool,
        }

        #[derive(Debug)]
        pub enum ClassAsciiKind {
            Word,
            // other variants omitted for brevity
        }
    }

    let mut formatter = MockFormatter::new();
    let ast = ast::ClassAscii { kind: ast::ClassAsciiKind::Word, negated: true };

    assert!(formatter.fmt_class_ascii(&ast).is_ok());
    assert_eq!(formatter.wtr.output, "[:^word:]");
}

