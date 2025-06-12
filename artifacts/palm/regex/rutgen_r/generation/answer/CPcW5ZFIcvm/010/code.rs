// Answer 0

#[derive(Debug)]
enum Ast {
    Empty(),
    Flags(),
    Literal(String),
    Dot(),
    Assertion(),
    Class(Class),
    Repetition(),
    Group(),
    Alternation(),
    Concat(),
}

#[derive(Debug)]
enum Class {
    Unicode(String),
    Perl(String),
    Bracketed(Vec<String>),
}

struct Visitor {
    depth: usize,
}

impl Visitor {
    fn new() -> Self {
        Visitor { depth: 0 }
    }

    fn decrement_depth(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }

    fn visit_post(&mut self, ast: &Ast) -> Result<(), String> {
        match *ast {
            Ast::Empty(_)
            | Ast::Flags(_)
            | Ast::Literal(_)
            | Ast::Dot(_)
            | Ast::Assertion(_)
            | Ast::Class(Class::Unicode(_))
            | Ast::Class(Class::Perl(_)) => {
                // These are all base cases, so we don't decrement depth.
                Ok(())
            }
            Ast::Class(Class::Bracketed(_))
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
fn test_visit_post_literal() {
    let mut visitor = Visitor::new();
    let ast = Ast::Literal("test".to_string());
    let result = visitor.visit_post(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_dot() {
    let mut visitor = Visitor::new();
    let ast = Ast::Dot();
    let result = visitor.visit_post(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_assertion() {
    let mut visitor = Visitor::new();
    let ast = Ast::Assertion();
    let result = visitor.visit_post(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_class_unicode() {
    let mut visitor = Visitor::new();
    let ast = Ast::Class(Class::Unicode("unicode".to_string()));
    let result = visitor.visit_post(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_flags() {
    let mut visitor = Visitor::new();
    let ast = Ast::Flags();
    let result = visitor.visit_post(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_empty() {
    let mut visitor = Visitor::new();
    let ast = Ast::Empty();
    let result = visitor.visit_post(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_class_perl() {
    let mut visitor = Visitor::new();
    let ast = Ast::Class(Class::Perl("perl".to_string()));
    let result = visitor.visit_post(&ast);
    assert_eq!(result, Ok(()));
}

