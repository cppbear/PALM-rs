// Answer 0

#[derive(Debug)]
struct DepthCounter {
    depth: usize,
}

impl DepthCounter {
    fn new() -> Self {
        DepthCounter { depth: 0 }
    }

    fn increment_depth(&mut self, _span: &str) -> Result<(), &'static str> {
        self.depth += 1;
        Ok(())
    }
}

#[derive(Debug)]
enum Ast {
    Empty(),
    Flags(),
    Literal(String),
    Dot(),
    Assertion(),
    Class(Class),
    Repetition(Repetition),
    Group(Group),
    Alternation(Alternation),
    Concat(Concat),
}

#[derive(Debug)]
enum Class {
    Unicode(String),
    Perl(String),
    Bracketed(Bracketed),
}

#[derive(Debug)]
struct Bracketed {
    span: String,
}

#[derive(Debug)]
struct Repetition {
    span: String,
}

#[derive(Debug)]
struct Group {
    span: String,
}

#[derive(Debug)]
struct Alternation {
    span: String,
}

#[derive(Debug)]
struct Concat {
    span: String,
}

type Result<T> = std::result::Result<T, &'static str>;

impl DepthCounter {
    fn visit_pre(&mut self, ast: &Ast) -> Result<()> {
        let span = match *ast {
            Ast::Empty(_)
            | Ast::Flags(_)
            | Ast::Literal(_)
            | Ast::Dot(_)
            | Ast::Assertion(_)
            | Ast::Class(Class::Unicode(_))
            | Ast::Class(Class::Perl(_)) => {
                // These are all base cases, so we don't increment depth.
                return Ok(());
            }
            Ast::Class(Class::Bracketed(ref x)) => &x.span,
            Ast::Repetition(ref x) => &x.span,
            Ast::Group(ref x) => &x.span,
            Ast::Alternation(ref x) => &x.span,
            Ast::Concat(ref x) => &x.span,
        };
        self.increment_depth(span)
    }
}

#[test]
fn test_visit_pre_with_literal() {
    let mut counter = DepthCounter::new();
    let ast = Ast::Literal("test".to_string());
    let result = counter.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_pre_with_unicode_class() {
    let mut counter = DepthCounter::new();
    let ast = Ast::Class(Class::Unicode("unicode_example".to_string()));
    let result = counter.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_pre_with_perl_class() {
    let mut counter = DepthCounter::new();
    let ast = Ast::Class(Class::Perl("perl_example".to_string()));
    let result = counter.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_pre_with_bracketed_class() {
    let mut counter = DepthCounter::new();
    let bracketed = Bracketed { span: "span_example".to_string() };
    let ast = Ast::Class(Class::Bracketed(bracketed));
    let result = counter.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_pre_with_repetition() {
    let mut counter = DepthCounter::new();
    let repetition = Repetition { span: "span_repetition".to_string() };
    let ast = Ast::Repetition(repetition);
    let result = counter.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

