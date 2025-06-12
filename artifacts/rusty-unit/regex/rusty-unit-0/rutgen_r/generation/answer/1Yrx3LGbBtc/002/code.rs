// Answer 0

#[test]
fn test_fmt_class_perl_word_non_negated() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct FmtPerl {
        wtr: MockWriter,
    }

    impl FmtPerl {
        fn new() -> Self {
            FmtPerl { wtr: MockWriter::new() }
        }

        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> std::fmt::Result {
            use ast::ClassPerlKind::*;
            match ast.kind {
                ast::ClassPerlKind::Digit if ast.negated => self.wtr.write_str(r"\D"),
                ast::ClassPerlKind::Digit => self.wtr.write_str(r"\d"),
                ast::ClassPerlKind::Space if ast.negated => self.wtr.write_str(r"\S"),
                ast::ClassPerlKind::Space => self.wtr.write_str(r"\s"),
                ast::ClassPerlKind::Word if ast.negated => self.wtr.write_str(r"\W"),
                ast::ClassPerlKind::Word => self.wtr.write_str(r"\w"),
            }
        }
    }

    struct ClassPerl {
        kind: ast::ClassPerlKind,
        negated: bool,
    }

    mod ast {
        pub struct ClassPerl {
            pub kind: ClassPerlKind,
            pub negated: bool,
        }

        #[derive(Clone, Copy)]
        pub enum ClassPerlKind {
            Digit,
            Space,
            Word,
        }
    }

    let mut fmt_perl = FmtPerl::new();
    let ast = ClassPerl {
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };

    let result = fmt_perl.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(fmt_perl.wtr.output, r"\w");
}

#[test]
fn test_fmt_class_perl_word_negated() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct FmtPerl {
        wtr: MockWriter,
    }

    impl FmtPerl {
        fn new() -> Self {
            FmtPerl { wtr: MockWriter::new() }
        }

        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> std::fmt::Result {
            use ast::ClassPerlKind::*;
            match ast.kind {
                ast::ClassPerlKind::Digit if ast.negated => self.wtr.write_str(r"\D"),
                ast::ClassPerlKind::Digit => self.wtr.write_str(r"\d"),
                ast::ClassPerlKind::Space if ast.negated => self.wtr.write_str(r"\S"),
                ast::ClassPerlKind::Space => self.wtr.write_str(r"\s"),
                ast::ClassPerlKind::Word if ast.negated => self.wtr.write_str(r"\W"),
                ast::ClassPerlKind::Word => self.wtr.write_str(r"\w"),
            }
        }
    }

    struct ClassPerl {
        kind: ast::ClassPerlKind,
        negated: bool,
    }

    mod ast {
        pub struct ClassPerl {
            pub kind: ClassPerlKind,
            pub negated: bool,
        }

        #[derive(Clone, Copy)]
        pub enum ClassPerlKind {
            Digit,
            Space,
            Word,
        }
    }

    let mut fmt_perl = FmtPerl::new();
    let ast = ClassPerl {
        kind: ast::ClassPerlKind::Word,
        negated: true,
    };

    let result = fmt_perl.fmt_class_perl(&ast);
    assert!(result.is_ok());
    assert_eq!(fmt_perl.wtr.output, r"\W");
}

