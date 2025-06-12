// Answer 0

#[test]
fn test_fmt_class_ascii_blank() {
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

    impl std::fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct ClassAscii {
        kind: ClassAsciiKind,
        negated: bool,
    }

    enum ClassAsciiKind {
        Blank,
    }

    struct Formatter {
        wtr: Writer,
    }

    impl Formatter {
        fn fmt_class_ascii(&mut self, ast: &ClassAscii) -> std::fmt::Result {
            use ClassAsciiKind::*;
            match ast.kind {
                Blank if ast.negated => self.wtr.write_str("[:^blank:]"),
                Blank => self.wtr.write_str("[:blank:]"),
                _ => Ok(()),
            }
        }
    }

    let mut formatter = Formatter {
        wtr: Writer::new(),
    };

    let ast = ClassAscii {
        kind: ClassAsciiKind::Blank,
        negated: false,
    };

    formatter.fmt_class_ascii(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "[:blank:]");
}

#[test]
fn test_fmt_class_ascii_blank_negated() {
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

    impl std::fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct ClassAscii {
        kind: ClassAsciiKind,
        negated: bool,
    }

    enum ClassAsciiKind {
        Blank,
    }

    struct Formatter {
        wtr: Writer,
    }

    impl Formatter {
        fn fmt_class_ascii(&mut self, ast: &ClassAscii) -> std::fmt::Result {
            use ClassAsciiKind::*;
            match ast.kind {
                Blank if ast.negated => self.wtr.write_str("[:^blank:]"),
                Blank => self.wtr.write_str("[:blank:]"),
                _ => Ok(()),
            }
        }
    }

    let mut formatter = Formatter {
        wtr: Writer::new(),
    };

    let ast = ClassAscii {
        kind: ClassAsciiKind::Blank,
        negated: true,
    };

    formatter.fmt_class_ascii(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "[:^blank:]");
}

