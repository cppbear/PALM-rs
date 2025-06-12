// Answer 0

#[derive(Debug)]
struct Ast {
    value: String,
}

enum Frame<'a> {
    Repetition { ast: &'a Ast },
    Group { ast: &'a Ast },
    Concat { head: &'a Ast },
    Alternation { head: &'a Ast },
}

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
fn test_child_group() {
    let ast_group = Ast { value: String::from("group_node") };
    let frame = Frame::Group { ast: &ast_group };

    let result = frame.child();
    assert_eq!(result.value, "group_node");
}

#[test]
fn test_child_repetition() {
    let ast_repetition = Ast { value: String::from("repetition_node") };
    let frame = Frame::Repetition { ast: &ast_repetition };

    let result = frame.child();
    assert_eq!(result.value, "repetition_node");
}

#[test]
fn test_child_concat() {
    let ast_concat = Ast { value: String::from("concat_node") };
    let frame = Frame::Concat { head: &ast_concat };

    let result = frame.child();
    assert_eq!(result.value, "concat_node");
}

#[test]
fn test_child_alternation() {
    let ast_alternation = Ast { value: String::from("alternation_node") };
    let frame = Frame::Alternation { head: &ast_alternation };

    let result = frame.child();
    assert_eq!(result.value, "alternation_node");
}

