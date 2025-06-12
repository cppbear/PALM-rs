// Answer 0

#[test]
fn test_child_with_class() {
    let head = Ast::Class(Class {});
    let tail = vec![Ast::Literal(Literal {})];
    let frame = Frame::Concat { head: &head, tail: &tail };
    let _ = frame.child();
}

#[test]
fn test_child_with_group() {
    let head = Ast::Group(Group { span: Span {}, kind: GroupKind {}, ast: Box::new(Ast::Empty(Span {})) });
    let tail = vec![Ast::Repetition(Repetition { span: Span {}, op: RepetitionOp {}, greedy: true, ast: Box::new(Ast::Dot(Span {})) })];
    let frame = Frame::Concat { head: &head, tail: &tail };
    let _ = frame.child();
}

#[test]
fn test_child_with_empty() {
    let head = Ast::Empty(Span {});
    let tail: Vec<Ast> = Vec::new();
    let frame = Frame::Concat { head: &head, tail: &tail };
    let _ = frame.child();
}

#[test]
fn test_child_with_dot() {
    let head = Ast::Dot(Span {});
    let tail = vec![Ast::Assertion(Assertion {})];
    let frame = Frame::Concat { head: &head, tail: &tail };
    let _ = frame.child();
}

#[test]
fn test_child_with_concat() {
    let head = Ast::Concat(Concat {});
    let tail = vec![Ast::Flags(SetFlags {})];
    let frame = Frame::Concat { head: &head, tail: &tail };
    let _ = frame.child();
}

