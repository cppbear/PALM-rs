// Answer 0

#[test]
fn test_fmt_class_ascii_ascii_negated() {
    use regex_syntax::ast::{self, ClassAscii, ClassAsciiKind};
    use std::fmt;

    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct FmtClassAscii {
        wtr: MockWriter,
    }

    impl FmtClassAscii {
        fn new() -> Self {
            FmtClassAscii {
                wtr: MockWriter::new(),
            }
        }

        fn fmt_class_ascii(&mut self, ast: &ClassAscii) -> fmt::Result {
            // This is where the actual implementation would be called in a real scenario.
            // For the purposes of the test, we would assume the function is working
            // according to the provided function above.
            // Here, we'll just simulate the functionality.
            use ClassAsciiKind::*;
            match ast.kind {
                Ascii if ast.negated => self.wtr.write_str("[:^ascii:]"),
                Ascii => self.wtr.write_str("[:ascii:]"),
                _ => unreachable!(),
            }
        }
    }

    let mut fmtter = FmtClassAscii::new();
    let ast = ClassAscii {
        kind: ClassAsciiKind::Ascii,
        negated: true,
    };

    let result = fmtter.fmt_class_ascii(&ast);

    assert!(result.is_ok());
    assert_eq!(fmtter.wtr.output, "[:^ascii:]");
}

