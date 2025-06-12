// Answer 0

#[test]
fn test_fmt_class_perl_digit_negated() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: Writer,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: Writer::new(),
            }
        }

        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
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
        kind: ast::ClassPerlKind,
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
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };
    assert_eq!(formatter.fmt_class_perl(&ast), Ok(()));
    assert_eq!(formatter.wtr.output, r"\D");
}

#[test]
fn test_fmt_class_perl_digit_not_negated() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: Writer,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: Writer::new(),
            }
        }

        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
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
        kind: ast::ClassPerlKind,
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
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    assert_eq!(formatter.fmt_class_perl(&ast), Ok(()));
    assert_eq!(formatter.wtr.output, r"\d");
}

#[test]
fn test_fmt_class_perl_space_negated() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: Writer,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: Writer::new(),
            }
        }

        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
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
        kind: ast::ClassPerlKind,
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
    assert_eq!(formatter.fmt_class_perl(&ast), Ok(()));
    assert_eq!(formatter.wtr.output, r"\S");
}

#[test]
fn test_fmt_class_perl_space_not_negated() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: Writer,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: Writer::new(),
            }
        }

        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
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
        kind: ast::ClassPerlKind,
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
    assert_eq!(formatter.fmt_class_perl(&ast), Ok(()));
    assert_eq!(formatter.wtr.output, r"\s");
}

#[test]
fn test_fmt_class_perl_word_negated() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: Writer,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: Writer::new(),
            }
        }

        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
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
        kind: ast::ClassPerlKind,
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
        kind: ast::ClassPerlKind::Word,
        negated: true,
    };
    assert_eq!(formatter.fmt_class_perl(&ast), Ok(()));
    assert_eq!(formatter.wtr.output, r"\W");
}

#[test]
fn test_fmt_class_perl_word_not_negated() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: Writer,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: Writer::new(),
            }
        }

        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
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
        kind: ast::ClassPerlKind,
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
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };
    assert_eq!(formatter.fmt_class_perl(&ast), Ok(()));
    assert_eq!(formatter.wtr.output, r"\w");
}

