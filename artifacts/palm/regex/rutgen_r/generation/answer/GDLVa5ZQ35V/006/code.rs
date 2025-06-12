// Answer 0

#[test]
fn test_visit_post_unicode_class() -> fmt::Result {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl TestVisitor<'_> {
        fn fmt_set_flags(&mut self, _flags: &Flags) -> fmt::Result {
            self.wtr.output.push_str("flags");
            Ok(())
        }

        fn fmt_class_unicode(&mut self, _class: &UnicodeClass) -> fmt::Result {
            self.wtr.output.push_str("UnicodeClass");
            Ok(())
        }
    }

    enum Ast {
        Flags(Flags),
        Class(Class),
    }

    enum Class {
        Unicode(UnicodeClass),
    }

    struct Flags;
    struct UnicodeClass;

    let mut writer = TestWriter::new();
    let mut visitor = TestVisitor { wtr: &mut writer };
    let ast = Ast::Class(Class::Unicode(UnicodeClass));

    visitor.visit_post(&ast)?;

    assert_eq!(writer.output, "UnicodeClass");
    Ok(())
}

#[test]
fn test_visit_post_bracketed_class() -> fmt::Result {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl TestVisitor<'_> {
        fn fmt_set_flags(&mut self, _flags: &Flags) -> fmt::Result {
            self.wtr.output.push_str("flags");
            Ok(())
        }

        fn fmt_class_bracketed_post(&mut self, _class: &BracketedClass) -> fmt::Result {
            self.wtr.output.push_str("BracketedClass");
            Ok(())
        }
    }

    enum Ast {
        Flags(Flags),
        Class(Class),
    }

    enum Class {
        Bracketed(BracketedClass),
    }

    struct Flags;
    struct BracketedClass;

    let mut writer = TestWriter::new();
    let mut visitor = TestVisitor { wtr: &mut writer };
    let ast = Ast::Class(Class::Bracketed(BracketedClass));

    visitor.visit_post(&ast)?;

    assert_eq!(writer.output, "BracketedClass");
    Ok(())
}

#[test]
fn test_visit_post_perl_class() -> fmt::Result {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl TestVisitor<'_> {
        fn fmt_set_flags(&mut self, _flags: &Flags) -> fmt::Result {
            self.wtr.output.push_str("flags");
            Ok(())
        }

        fn fmt_class_perl(&mut self, _class: &PerlClass) -> fmt::Result {
            self.wtr.output.push_str("PerlClass");
            Ok(())
        }
    }

    enum Ast {
        Flags(Flags),
        Class(Class),
    }

    enum Class {
        Perl(PerlClass),
    }

    struct Flags;
    struct PerlClass;

    let mut writer = TestWriter::new();
    let mut visitor = TestVisitor { wtr: &mut writer };
    let ast = Ast::Class(Class::Perl(PerlClass));

    visitor.visit_post(&ast)?;

    assert_eq!(writer.output, "PerlClass");
    Ok(())
}

#[test]
fn test_visit_post_flags() -> fmt::Result {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl TestVisitor<'_> {
        fn fmt_set_flags(&mut self, _flags: &Flags) -> fmt::Result {
            self.wtr.output.push_str("flags");
            Ok(())
        }

        fn fmt_literal(&mut self, _literal: &str) -> fmt::Result {
            self.wtr.output.push_str("literal");
            Ok(())
        }
    }

    enum Ast {
        Flags(Flags),
        Literal(String),
    }

    struct Flags;

    let mut writer = TestWriter::new();
    let mut visitor = TestVisitor { wtr: &mut writer };
    let ast = Ast::Flags(Flags);

    visitor.visit_post(&ast)?;

    assert_eq!(writer.output, "flags");
    Ok(())
}

