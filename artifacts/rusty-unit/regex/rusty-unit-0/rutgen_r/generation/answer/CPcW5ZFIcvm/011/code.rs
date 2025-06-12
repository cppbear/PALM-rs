// Answer 0

#[test]
fn test_visit_post_with_flags() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }

        fn visit_post(&mut self, ast: &Ast) -> Result<()> {
            match *ast {
                Ast::Empty(_)
                | Ast::Flags(_)
                | Ast::Literal(_)
                | Ast::Dot(_)
                | Ast::Assertion(_)
                | Ast::Class(ast::Class::Unicode(_))
                | Ast::Class(ast::Class::Perl(_)) => {
                    Ok(())
                }
                Ast::Class(ast::Class::Bracketed(_))
                | Ast::Repetition(_)
                | Ast::Group(_)
                | Ast::Alternation(_)
                | Ast::Concat(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 10 };

    let ast = Ast::Flags(ast::Flags::new());
    assert_eq!(visitor.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_with_literal() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }

        fn visit_post(&mut self, ast: &Ast) -> Result<()> {
            match *ast {
                Ast::Empty(_)
                | Ast::Flags(_)
                | Ast::Literal(_)
                | Ast::Dot(_)
                | Ast::Assertion(_)
                | Ast::Class(ast::Class::Unicode(_))
                | Ast::Class(ast::Class::Perl(_)) => {
                    Ok(())
                }
                Ast::Class(ast::Class::Bracketed(_))
                | Ast::Repetition(_)
                | Ast::Group(_)
                | Ast::Alternation(_)
                | Ast::Concat(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 10 };

    let ast = Ast::Literal(ast::Literal::new("test"));
    assert_eq!(visitor.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_with_unicode_class() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }

        fn visit_post(&mut self, ast: &Ast) -> Result<()> {
            match *ast {
                Ast::Empty(_)
                | Ast::Flags(_)
                | Ast::Literal(_)
                | Ast::Dot(_)
                | Ast::Assertion(_)
                | Ast::Class(ast::Class::Unicode(_))
                | Ast::Class(ast::Class::Perl(_)) => {
                    Ok(())
                }
                Ast::Class(ast::Class::Bracketed(_))
                | Ast::Repetition(_)
                | Ast::Group(_)
                | Ast::Alternation(_)
                | Ast::Concat(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 10 };

    let ast = Ast::Class(ast::Class::Unicode(ast::UnicodeClass::new()));
    assert_eq!(visitor.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_with_dot() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }

        fn visit_post(&mut self, ast: &Ast) -> Result<()> {
            match *ast {
                Ast::Empty(_)
                | Ast::Flags(_)
                | Ast::Literal(_)
                | Ast::Dot(_)
                | Ast::Assertion(_)
                | Ast::Class(ast::Class::Unicode(_))
                | Ast::Class(ast::Class::Perl(_)) => {
                    Ok(())
                }
                Ast::Class(ast::Class::Bracketed(_))
                | Ast::Repetition(_)
                | Ast::Group(_)
                | Ast::Alternation(_)
                | Ast::Concat(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 10 };

    let ast = Ast::Dot(ast::Dot::new());
    assert_eq!(visitor.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_with_assertion() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }

        fn visit_post(&mut self, ast: &Ast) -> Result<()> {
            match *ast {
                Ast::Empty(_)
                | Ast::Flags(_)
                | Ast::Literal(_)
                | Ast::Dot(_)
                | Ast::Assertion(_)
                | Ast::Class(ast::Class::Unicode(_))
                | Ast::Class(ast::Class::Perl(_)) => {
                    Ok(())
                }
                Ast::Class(ast::Class::Bracketed(_))
                | Ast::Repetition(_)
                | Ast::Group(_)
                | Ast::Alternation(_)
                | Ast::Concat(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 10 };

    let ast = Ast::Assertion(ast::Assertion::new());
    assert_eq!(visitor.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_with_perl_class() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }

        fn visit_post(&mut self, ast: &Ast) -> Result<()> {
            match *ast {
                Ast::Empty(_)
                | Ast::Flags(_)
                | Ast::Literal(_)
                | Ast::Dot(_)
                | Ast::Assertion(_)
                | Ast::Class(ast::Class::Unicode(_))
                | Ast::Class(ast::Class::Perl(_)) => {
                    Ok(())
                }
                Ast::Class(ast::Class::Bracketed(_))
                | Ast::Repetition(_)
                | Ast::Group(_)
                | Ast::Alternation(_)
                | Ast::Concat(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 10 };

    let ast = Ast::Class(ast::Class::Perl(ast::PerlClass::new()));
    assert_eq!(visitor.visit_post(&ast), Ok(()));
}

