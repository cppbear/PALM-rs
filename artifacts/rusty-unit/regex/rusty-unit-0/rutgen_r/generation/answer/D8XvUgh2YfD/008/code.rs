// Answer 0

#[test]
fn test_visit_pre_assertion() {
    struct Visitor {
        depth: usize,
    }

    impl Visitor {
        fn increment_depth(&mut self, _span: &str) -> Result<()> {
            self.depth += 1;
            Ok(())
        }

        fn visit_pre(&mut self, ast: &Ast) -> Result<()> {
            let span = match *ast {
                Ast::Empty(_)
                | Ast::Flags(_)
                | Ast::Literal(_)
                | Ast::Dot(_)
                | Ast::Assertion(_)
                | Ast::Class(ast::Class::Unicode(_))
                | Ast::Class(ast::Class::Perl(_)) => {
                    return Ok(());
                }
                Ast::Class(ast::Class::Bracketed(ref x)) => &x.span,
                Ast::Repetition(ref x) => &x.span,
                Ast::Group(ref x) => &x.span,
                Ast::Alternation(ref x) => &x.span,
                Ast::Concat(ref x) => &x.span,
            };
            self.increment_depth(span)
        }
    }

    enum Ast {
        Empty(()),
        Flags(()),
        Literal(()),
        Dot(()),
        Assertion(()),
        Class(Class),
        Group(Box<Ast>),
        Repetition(Box<Ast>),
        Alternation(Vec<Ast>),
        Concat(Vec<Ast>),
    }

    enum Class {
        Unicode(()),
        Perl(())
    }

    let mut visitor = Visitor { depth: 0 };
    let ast = Ast::Assertion(()); // Meets the constraint of Ast::Assertion(_)

    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_pre_dot() {
    struct Visitor {
        depth: usize,
    }

    impl Visitor {
        fn increment_depth(&mut self, _span: &str) -> Result<()> {
            self.depth += 1;
            Ok(())
        }

        fn visit_pre(&mut self, ast: &Ast) -> Result<()> {
            let span = match *ast {
                Ast::Empty(_)
                | Ast::Flags(_)
                | Ast::Literal(_)
                | Ast::Dot(_)
                | Ast::Assertion(_)
                | Ast::Class(ast::Class::Unicode(_))
                | Ast::Class(ast::Class::Perl(_)) => {
                    return Ok(());
                }
                Ast::Class(ast::Class::Bracketed(ref x)) => &x.span,
                Ast::Repetition(ref x) => &x.span,
                Ast::Group(ref x) => &x.span,
                Ast::Alternation(ref x) => &x.span,
                Ast::Concat(ref x) => &x.span,
            };
            self.increment_depth(span)
        }
    }

    enum Ast {
        Empty(()),
        Flags(()),
        Literal(()),
        Dot(()),
        Assertion(()),
        Class(Class),
        Group(Box<Ast>),
        Repetition(Box<Ast>),
        Alternation(Vec<Ast>),
        Concat(Vec<Ast>),
    }

    enum Class {
        Unicode(()),
        Perl(())
    }

    let mut visitor = Visitor { depth: 0 };
    let ast = Ast::Dot(()); // Meets the constraint of Ast::Dot(_)

    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_pre_flags() {
    struct Visitor {
        depth: usize,
    }

    impl Visitor {
        fn increment_depth(&mut self, _span: &str) -> Result<()> {
            self.depth += 1;
            Ok(())
        }

        fn visit_pre(&mut self, ast: &Ast) -> Result<()> {
            let span = match *ast {
                Ast::Empty(_)
                | Ast::Flags(_)
                | Ast::Literal(_)
                | Ast::Dot(_)
                | Ast::Assertion(_)
                | Ast::Class(ast::Class::Unicode(_))
                | Ast::Class(ast::Class::Perl(_)) => {
                    return Ok(());
                }
                Ast::Class(ast::Class::Bracketed(ref x)) => &x.span,
                Ast::Repetition(ref x) => &x.span,
                Ast::Group(ref x) => &x.span,
                Ast::Alternation(ref x) => &x.span,
                Ast::Concat(ref x) => &x.span,
            };
            self.increment_depth(span)
        }
    }

    enum Ast {
        Empty(()),
        Flags(()),
        Literal(()),
        Dot(()),
        Assertion(()),
        Class(Class),
        Group(Box<Ast>),
        Repetition(Box<Ast>),
        Alternation(Vec<Ast>),
        Concat(Vec<Ast>),
    }

    enum Class {
        Unicode(()),
        Perl(())
    }

    let mut visitor = Visitor { depth: 0 };
    let ast = Ast::Flags(()); // Meets the constraint of Ast::Flags(_)

    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_pre_literal() {
    struct Visitor {
        depth: usize,
    }

    impl Visitor {
        fn increment_depth(&mut self, _span: &str) -> Result<()> {
            self.depth += 1;
            Ok(())
        }

        fn visit_pre(&mut self, ast: &Ast) -> Result<()> {
            let span = match *ast {
                Ast::Empty(_)
                | Ast::Flags(_)
                | Ast::Literal(_)
                | Ast::Dot(_)
                | Ast::Assertion(_)
                | Ast::Class(ast::Class::Unicode(_))
                | Ast::Class(ast::Class::Perl(_)) => {
                    return Ok(());
                }
                Ast::Class(ast::Class::Bracketed(ref x)) => &x.span,
                Ast::Repetition(ref x) => &x.span,
                Ast::Group(ref x) => &x.span,
                Ast::Alternation(ref x) => &x.span,
                Ast::Concat(ref x) => &x.span,
            };
            self.increment_depth(span)
        }
    }

    enum Ast {
        Empty(()),
        Flags(()),
        Literal(()),
        Dot(()),
        Assertion(()),
        Class(Class),
        Group(Box<Ast>),
        Repetition(Box<Ast>),
        Alternation(Vec<Ast>),
        Concat(Vec<Ast>),
    }

    enum Class {
        Unicode(()),
        Perl(())
    }

    let mut visitor = Visitor { depth: 0 };
    let ast = Ast::Literal(()); // Meets the constraint of Ast::Literal(_)

    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

