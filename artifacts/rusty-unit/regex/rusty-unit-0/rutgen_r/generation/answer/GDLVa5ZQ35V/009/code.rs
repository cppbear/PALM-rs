// Answer 0

#[test]
fn test_visit_post_dot() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockVisitor {
        wtr: MockWriter,
    }

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor {
                wtr: MockWriter::new(),
            }
        }

        fn visit_post(&mut self, ast: &Ast) -> fmt::Result {
            // Implementation here
            match *ast {
                Ast::Empty(_) => Ok(()),
                Ast::Flags(ref x) => self.fmt_set_flags(x),
                Ast::Literal(ref x) => self.fmt_literal(x),
                Ast::Dot(_) => self.wtr.write_str("."),
                Ast::Assertion(ref x) => self.fmt_assertion(x),
                Ast::Class(Class::Perl(ref x)) => self.fmt_class_perl(x),
                Ast::Class(Class::Unicode(ref x)) => self.fmt_class_unicode(x),
                Ast::Class(Class::Bracketed(ref x)) => self.fmt_class_bracketed_post(x),
                Ast::Repetition(ref x) => self.fmt_repetition(x),
                Ast::Group(ref x) => self.fmt_group_post(x),
                Ast::Alternation(_) => Ok(()),
                Ast::Concat(_) => Ok(()),
            }
        }

        fn fmt_set_flags(&self, _: &Flags) -> fmt::Result { Ok(()) }
        fn fmt_literal(&self, _: &Literal) -> fmt::Result { Ok(()) }
        fn fmt_assertion(&self, _: &Assertion) -> fmt::Result { Ok(()) }
        fn fmt_class_perl(&self, _: &PerlClass) -> fmt::Result { Ok(()) }
        fn fmt_class_unicode(&self, _: &UnicodeClass) -> fmt::Result { Ok(()) }
        fn fmt_class_bracketed_post(&self, _: &BracketedClass) -> fmt::Result { Ok(()) }
        fn fmt_repetition(&self, _: &Repetition) -> fmt::Result { Ok(()) }
        fn fmt_group_post(&self, _: &Group) -> fmt::Result { Ok(()) }
    }

    let mut visitor = MockVisitor::new();
    let ast_dot = Ast::Dot(());
    let result = visitor.visit_post(&ast_dot);
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, ".");
}

