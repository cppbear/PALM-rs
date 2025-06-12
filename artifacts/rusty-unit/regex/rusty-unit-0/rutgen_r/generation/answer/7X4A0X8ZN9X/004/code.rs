// Answer 0

#[derive(Debug)]
struct Repetition {
    ast: Ast,
}

#[derive(Debug)]
struct Group {
    ast: Ast,
}

#[derive(Debug)]
struct Concat {
    head: &'static Ast,
}

#[derive(Debug)]
struct Alternation {
    head: &'static Ast,
}

#[derive(Debug)]
enum Frame<'a> {
    Repetition(Repetition),
    Group(Group),
    Concat { head: &'a Ast, tail: &'a [Ast] },
    Alternation { head: &'a Ast, tail: &'a [Ast] },
}

#[derive(Debug)]
struct Ast;

#[test]
fn test_child_from_repetition() {
    let ast_instance = Ast;
    let repetition_frame = Frame::Repetition(Repetition { ast: ast_instance });

    assert_eq!(repetition_frame.child(), &ast_instance);
}

#[test]
fn test_child_from_group() {
    let ast_instance = Ast;
    let group_frame = Frame::Group(Group { ast: ast_instance });

    assert_eq!(group_frame.child(), &ast_instance);
}

#[test]
fn test_child_from_concat() {
    let ast_instance = Ast;
    let concat_frame = Frame::Concat { head: &ast_instance, tail: &[] };

    assert_eq!(concat_frame.child(), &ast_instance);
}

#[test]
fn test_child_from_alternation() {
    let ast_instance = Ast;
    let alternation_frame = Frame::Alternation { head: &ast_instance, tail: &[] };

    assert_eq!(alternation_frame.child(), &ast_instance);
}

