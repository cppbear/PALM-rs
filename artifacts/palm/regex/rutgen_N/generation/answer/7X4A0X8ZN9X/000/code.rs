// Answer 0

#[derive(Debug)]
enum Frame<'a> {
    Repetition { ast: &'a Ast },
    Group { ast: &'a Ast },
    Concat { head: &'a Ast },
    Alternation { head: &'a Ast },
}

#[derive(Debug)]
struct Ast;

impl<'a> Frame<'a> {
    fn child(&self) -> &'a Ast {
        match *self {
            Frame::Repetition { ast } => ast,
            Frame::Group { ast } => ast,
            Frame::Concat { head } => head,
            Frame::Alternation { head } => head,
        }
    }
}

#[test]
fn test_child_repetition() {
    let ast = Ast;
    let frame = Frame::Repetition { ast: &ast };
    assert_eq!(frame.child(), &ast);
}

#[test]
fn test_child_group() {
    let ast = Ast;
    let frame = Frame::Group { ast: &ast };
    assert_eq!(frame.child(), &ast);
}

#[test]
fn test_child_concat() {
    let ast = Ast;
    let frame = Frame::Concat { head: &ast };
    assert_eq!(frame.child(), &ast);
}

#[test]
fn test_child_alternation() {
    let ast = Ast;
    let frame = Frame::Alternation { head: &ast };
    assert_eq!(frame.child(), &ast);
}

