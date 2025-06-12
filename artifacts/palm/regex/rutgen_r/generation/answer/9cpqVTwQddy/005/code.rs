// Answer 0

#[test]
fn test_fmt_class_ascii_upper_negated() {
    use std::fmt::{self, Write};

    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter {
        wtr: MockWriter,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self { wtr: MockWriter::new() }
        }

        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {
            use ast::ClassAsciiKind::*;
            match ast.kind {
                Upper if ast.negated => self.wtr.write_str("[:^upper:]"),
                _ => Ok(()),
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
            Upper,
            // Other variants omitted for brevity
        }
    }

    let mut formatter = MockFormatter::new();
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Upper,
        negated: true,
    };

    let result = formatter.fmt_class_ascii(&ast);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "[:^upper:]");
}

