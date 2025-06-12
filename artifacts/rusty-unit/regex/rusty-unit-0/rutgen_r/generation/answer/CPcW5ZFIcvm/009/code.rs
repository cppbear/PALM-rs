// Answer 0

#[test]
fn test_visit_post_empty() {
    struct Visitor {
        depth: usize,
    }

    impl Visitor {
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }

        fn visit_post(&mut self, ast: &Ast) -> Result<()> {
            match *ast {
                Ast::Empty(_) => Ok(()),
                Ast::Flags(_) => Ok(()),
                Ast::Literal(_) => Ok(()),
                Ast::Dot(_) => Ok(()),
                Ast::Assertion(_) => Ok(()),
                Ast::Class(ast::Class::Unicode(_)) => Ok(()),
                Ast::Class(ast::Class::Perl(_)) => Ok(()),
                Ast::Class(ast::Class::Bracketed(_)) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Repetition(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Group(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Alternation(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Concat(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = Visitor { depth: 1 };
    let ast = Ast::Empty(Default::default());
    assert_eq!(visitor.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_literal() {
    struct Visitor {
        depth: usize,
    }

    impl Visitor {
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }

        fn visit_post(&mut self, ast: &Ast) -> Result<()> {
            match *ast {
                Ast::Empty(_) => Ok(()),
                Ast::Flags(_) => Ok(()),
                Ast::Literal(_) => Ok(()),
                Ast::Dot(_) => Ok(()),
                Ast::Assertion(_) => Ok(()),
                Ast::Class(ast::Class::Unicode(_)) => Ok(()),
                Ast::Class(ast::Class::Perl(_)) => Ok(()),
                Ast::Class(ast::Class::Bracketed(_)) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Repetition(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Group(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Alternation(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Concat(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = Visitor { depth: 1 };
    let ast = Ast::Literal(Default::default());
    assert_eq!(visitor.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_dot() {
    struct Visitor {
        depth: usize,
    }

    impl Visitor {
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }

        fn visit_post(&mut self, ast: &Ast) -> Result<()> {
            match *ast {
                Ast::Empty(_) => Ok(()),
                Ast::Flags(_) => Ok(()),
                Ast::Literal(_) => Ok(()),
                Ast::Dot(_) => Ok(()),
                Ast::Assertion(_) => Ok(()),
                Ast::Class(ast::Class::Unicode(_)) => Ok(()),
                Ast::Class(ast::Class::Perl(_)) => Ok(()),
                Ast::Class(ast::Class::Bracketed(_)) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Repetition(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Group(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Alternation(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Concat(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = Visitor { depth: 1 };
    let ast = Ast::Dot(Default::default());
    assert_eq!(visitor.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_assertion() {
    struct Visitor {
        depth: usize,
    }

    impl Visitor {
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }

        fn visit_post(&mut self, ast: &Ast) -> Result<()> {
            match *ast {
                Ast::Empty(_) => Ok(()),
                Ast::Flags(_) => Ok(()),
                Ast::Literal(_) => Ok(()),
                Ast::Dot(_) => Ok(()),
                Ast::Assertion(_) => Ok(()),
                Ast::Class(ast::Class::Unicode(_)) => Ok(()),
                Ast::Class(ast::Class::Perl(_)) => Ok(()),
                Ast::Class(ast::Class::Bracketed(_)) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Repetition(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Group(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Alternation(_) => {
                    self.decrement_depth();
                    Ok(())
                }
                Ast::Concat(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = Visitor { depth: 1 };
    let ast = Ast::Assertion(Default::default());
    assert_eq!(visitor.visit_post(&ast), Ok(()));
}

