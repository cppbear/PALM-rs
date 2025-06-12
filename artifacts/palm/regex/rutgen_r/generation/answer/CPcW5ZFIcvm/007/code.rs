// Answer 0

fn visit_post_tests() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
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
                    // These are all base cases, so we don't decrement depth.
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

    #[test]
    fn test_visit_post_empty() {
        let mut visitor = TestVisitor { depth: 1 };
        let ast = Ast::Empty(());
        assert_eq!(visitor.visit_post(&ast), Ok(()));
    }

    #[test]
    fn test_visit_post_unicode_class() {
        let mut visitor = TestVisitor { depth: 1 };
        let ast = Ast::Class(ast::Class::Unicode(())); 
        assert_eq!(visitor.visit_post(&ast), Ok(()));
    }

    #[test]
    fn test_visit_post_perl_class() {
        let mut visitor = TestVisitor { depth: 1 };
        let ast = Ast::Class(ast::Class::Perl(()));
        assert_eq!(visitor.visit_post(&ast), Ok(()));
    }

    #[test]
    fn test_visit_post_literal() {
        let mut visitor = TestVisitor { depth: 1 };
        let ast = Ast::Literal("a");
        assert_eq!(visitor.visit_post(&ast), Ok(()));
    }

    #[test]
    fn test_visit_post_dot() {
        let mut visitor = TestVisitor { depth: 1 };
        let ast = Ast::Dot(());
        assert_eq!(visitor.visit_post(&ast), Ok(()));
    }

    #[test]
    fn test_visit_post_assertion() {
        let mut visitor = TestVisitor { depth: 1 };
        let ast = Ast::Assertion(());
        assert_eq!(visitor.visit_post(&ast), Ok(()));
    }

    #[test]
    fn test_visit_post_flags() {
        let mut visitor = TestVisitor { depth: 1 };
        let ast = Ast::Flags(());
        assert_eq!(visitor.visit_post(&ast), Ok(()));
    }

    #[test]
    fn test_visit_post_bracketed_class() {
        let mut visitor = TestVisitor { depth: 1 };
        let ast = Ast::Class(ast::Class::Bracketed(()));
        assert_eq!(visitor.visit_post(&ast), Ok(()));
    }
}

