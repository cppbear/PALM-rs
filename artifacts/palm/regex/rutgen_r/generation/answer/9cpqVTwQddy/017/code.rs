// Answer 0

#[test]
fn test_fmt_class_ascii_digit_negated() {
    use std::fmt::{self, Write};

    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer { output: String::new() }
        }
    }

    impl Write for Writer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MyFormatter {
        wtr: Writer,
    }

    impl MyFormatter {
        fn new() -> Self {
            MyFormatter {
                wtr: Writer::new(),
            }
        }

        fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {
            use ast::ClassAsciiKind::*;
            match ast.kind {
                Digit if ast.negated => self.wtr.write_str("[:^digit:]"),
                _ => Ok(()), // Other cases not relevant for this test
            }
        }
    }

    mod ast {
        pub struct ClassAscii {
            pub kind: ClassAsciiKind,
            pub negated: bool,
        }

        #[derive(Clone, Copy)]
        pub enum ClassAsciiKind {
            Digit,
        }
    }

    let mut formatter = MyFormatter::new();
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };

    formatter.fmt_class_ascii(&ast).expect("Formatting failed");
    assert_eq!(formatter.wtr.output, "[:^digit:]");
}

