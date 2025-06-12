// Answer 0

#[test]
fn test_visit_post_with_ast_alternation() {
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

    enum Ast {
        Empty(()),
        Flags(()),
        Literal(()),
        Dot(()),
        Assertion(()),
        Class(Class),
        Repetition(()),
        Group(()),
        Alternation(()),
        Concat(()),
    }

    enum Class {
        Unicode(()),
        Perl(()),
        Bracketed(()),
    }

    type Result<T> = std::result::Result<T, std::io::Error>;

    let mut visitor = TestVisitor { depth: 1 };
    
    let ast_input = Ast::Alternation(());
    let result = visitor.visit_post(&ast_input);
    
    assert_eq!(result, Ok(()));
}

