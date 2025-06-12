// Answer 0

#[test]
fn test_visit_post_class_unicode() {
    use regex_syntax::ast::{Ast, Class};
    use std::fmt::{self, Write};

    struct TestVisitor {
        result: String,
    }

    impl TestVisitor {
        fn fmt_set_flags(&mut self, _flags: &()) -> fmt::Result {
            Ok(())
        }

        fn fmt_literal(&mut self, _literal: &str) -> fmt::Result {
            self.result.push_str(_literal);
            Ok(())
        }

        fn fmt_assertion(&mut self, _assertion: &()) -> fmt::Result {
            Ok(())
        }

        fn fmt_class_perl(&mut self, _class: &()) -> fmt::Result {
            self.result.push_str("class_perl");
            Ok(())
        }

        fn fmt_class_unicode(&mut self, _class: &()) -> fmt::Result {
            self.result.push_str("class_unicode");
            Ok(())
        }

        fn fmt_class_bracketed_post(&mut self, _class: &()) -> fmt::Result {
            self.result.push_str("class_bracketed");
            Ok(())
        }

        fn fmt_repetition(&mut self, _repetition: &()) -> fmt::Result {
            Ok(())
        }

        fn fmt_group_post(&mut self, _group: &()) -> fmt::Result {
            Ok(())
        }

        fn visit_post(&mut self, ast: &Ast) -> fmt::Result {
            match *ast {
                Ast::Empty(_) => Ok(()),
                Ast::Flags(ref x) => self.fmt_set_flags(x),
                Ast::Literal(ref x) => self.fmt_literal(x),
                Ast::Dot(_) => self.result.write_str("."),
                Ast::Assertion(ref x) => self.fmt_assertion(x),
                Ast::Class(Class::Perl(ref x)) => self.fmt_class_perl(x),
                Ast::Class(Class::Unicode(ref x)) => self.fmt_class_unicode(x),
                Ast::Class(Class::Bracketed(ref x)) => {
                    self.fmt_class_bracketed_post(x)
                }
                Ast::Repetition(ref x) => self.fmt_repetition(x),
                Ast::Group(ref x) => self.fmt_group_post(x),
                Ast::Alternation(_) => Ok(()),
                Ast::Concat(_) => Ok(()),
            }
        }
    }

    let mut visitor = TestVisitor { result: String::new() };
    let ast = Ast::Class(Class::Unicode(&()));
    visitor.visit_post(&ast).expect("visit_post failed");
    assert_eq!(visitor.result, "class_unicode");
}

#[test]
fn test_visit_post_class_bracketed() {
    use regex_syntax::ast::{Ast, Class};
    use std::fmt::{self, Write};

    struct TestVisitor {
        result: String,
    }

    impl TestVisitor {
        fn fmt_set_flags(&mut self, _flags: &()) -> fmt::Result {
            Ok(())
        }

        fn fmt_literal(&mut self, _literal: &str) -> fmt::Result {
            self.result.push_str(_literal);
            Ok(())
        }

        fn fmt_assertion(&mut self, _assertion: &()) -> fmt::Result {
            Ok(())
        }

        fn fmt_class_perl(&mut self, _class: &()) -> fmt::Result {
            self.result.push_str("class_perl");
            Ok(())
        }

        fn fmt_class_unicode(&mut self, _class: &()) -> fmt::Result {
            self.result.push_str("class_unicode");
            Ok(())
        }

        fn fmt_class_bracketed_post(&mut self, _class: &()) -> fmt::Result {
            self.result.push_str("class_bracketed");
            Ok(())
        }

        fn fmt_repetition(&mut self, _repetition: &()) -> fmt::Result {
            Ok(())
        }

        fn fmt_group_post(&mut self, _group: &()) -> fmt::Result {
            Ok(())
        }

        fn visit_post(&mut self, ast: &Ast) -> fmt::Result {
            match *ast {
                Ast::Empty(_) => Ok(()),
                Ast::Flags(ref x) => self.fmt_set_flags(x),
                Ast::Literal(ref x) => self.fmt_literal(x),
                Ast::Dot(_) => self.result.write_str("."),
                Ast::Assertion(ref x) => self.fmt_assertion(x),
                Ast::Class(Class::Perl(ref x)) => self.fmt_class_perl(x),
                Ast::Class(Class::Unicode(ref x)) => self.fmt_class_unicode(x),
                Ast::Class(Class::Bracketed(ref x)) => {
                    self.fmt_class_bracketed_post(x)
                }
                Ast::Repetition(ref x) => self.fmt_repetition(x),
                Ast::Group(ref x) => self.fmt_group_post(x),
                Ast::Alternation(_) => Ok(()),
                Ast::Concat(_) => Ok(()),
            }
        }
    }

    let mut visitor = TestVisitor { result: String::new() };
    let ast = Ast::Class(Class::Bracketed(&()));
    visitor.visit_post(&ast).expect("visit_post failed");
    assert_eq!(visitor.result, "class_bracketed");
}

#[test]
fn test_visit_post_literal() {
    use regex_syntax::ast::{Ast, Class};
    use std::fmt::{self, Write};

    struct TestVisitor {
        result: String,
    }

    impl TestVisitor {
        fn fmt_set_flags(&mut self, _flags: &()) -> fmt::Result {
            Ok(())
        }

        fn fmt_literal(&mut self, _literal: &str) -> fmt::Result {
            self.result.push_str(_literal);
            Ok(())
        }

        fn fmt_assertion(&mut self, _assertion: &()) -> fmt::Result {
            Ok(())
        }

        fn fmt_class_perl(&mut self, _class: &()) -> fmt::Result {
            self.result.push_str("class_perl");
            Ok(())
        }

        fn fmt_class_unicode(&mut self, _class: &()) -> fmt::Result {
            self.result.push_str("class_unicode");
            Ok(())
        }

        fn fmt_class_bracketed_post(&mut self, _class: &()) -> fmt::Result {
            self.result.push_str("class_bracketed");
            Ok(())
        }

        fn fmt_repetition(&mut self, _repetition: &()) -> fmt::Result {
            Ok(())
        }

        fn fmt_group_post(&mut self, _group: &()) -> fmt::Result {
            Ok(())
        }

        fn visit_post(&mut self, ast: &Ast) -> fmt::Result {
            match *ast {
                Ast::Empty(_) => Ok(()),
                Ast::Flags(ref x) => self.fmt_set_flags(x),
                Ast::Literal(ref x) => self.fmt_literal(x),
                Ast::Dot(_) => self.result.write_str("."),
                Ast::Assertion(ref x) => self.fmt_assertion(x),
                Ast::Class(Class::Perl(ref x)) => self.fmt_class_perl(x),
                Ast::Class(Class::Unicode(ref x)) => self.fmt_class_unicode(x),
                Ast::Class(Class::Bracketed(ref x)) => {
                    self.fmt_class_bracketed_post(x)
                }
                Ast::Repetition(ref x) => self.fmt_repetition(x),
                Ast::Group(ref x) => self.fmt_group_post(x),
                Ast::Alternation(_) => Ok(()),
                Ast::Concat(_) => Ok(()),
            }
        }
    }

    let mut visitor = TestVisitor { result: String::new() };
    let ast = Ast::Literal("test_literal");
    visitor.visit_post(&ast).expect("visit_post failed");
    assert_eq!(visitor.result, "test_literal");
}

