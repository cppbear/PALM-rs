// Answer 0

#[test]
fn test_fmt_class_perl_space_negated() {
    struct MockWriter {
        written: String,
    }

    impl MockWriter {
        fn new() -> Self {
            Self {
                written: String::new(),
            }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.written.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: MockWriter,
    }

    impl Formatter {
        fn new() -> Self {
            Self { wtr: MockWriter::new() }
        }

        fn fmt_class_perl(&mut self, ast: &ClassPerl) -> fmt::Result {
            use ast::ClassPerlKind::*;
            match ast.kind {
                Digit if ast.negated => self.wtr.write_str(r"\D"),
                Digit => self.wtr.write_str(r"\d"),
                Space if ast.negated => self.wtr.write_str(r"\S"),
                Space => self.wtr.write_str(r"\s"),
                Word if ast.negated => self.wtr.write_str(r"\W"),
                Word => self.wtr.write_str(r"\w"),
            }
        }
    }

    struct ClassPerl {
        kind: ClassPerlKind,
        negated: bool,
    }

    mod ast {
        pub enum ClassPerlKind {
            Digit,
            Space,
            Word,
        }
    }

    let mut formatter = Formatter::new();
    let ast = ClassPerl {
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };

    let result = formatter.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.written, r"\S");
}

#[test]
fn test_fmt_class_perl_space() {
    struct MockWriter {
        written: String,
    }

    impl MockWriter {
        fn new() -> Self {
            Self {
                written: String::new(),
            }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.written.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: MockWriter,
    }

    impl Formatter {
        fn new() -> Self {
            Self { wtr: MockWriter::new() }
        }

        fn fmt_class_perl(&mut self, ast: &ClassPerl) -> fmt::Result {
            use ast::ClassPerlKind::*;
            match ast.kind {
                Digit if ast.negated => self.wtr.write_str(r"\D"),
                Digit => self.wtr.write_str(r"\d"),
                Space if ast.negated => self.wtr.write_str(r"\S"),
                Space => self.wtr.write_str(r"\s"),
                Word if ast.negated => self.wtr.write_str(r"\W"),
                Word => self.wtr.write_str(r"\w"),
            }
        }
    }

    struct ClassPerl {
        kind: ClassPerlKind,
        negated: bool,
    }

    mod ast {
        pub enum ClassPerlKind {
            Digit,
            Space,
            Word,
        }
    }

    let mut formatter = Formatter::new();
    let ast = ClassPerl {
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };

    let result = formatter.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.written, r"\s");
}

