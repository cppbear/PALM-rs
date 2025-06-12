// Answer 0

#[derive(Debug)]
enum Frame<'a> {
    Repetition(&'a str),
    Group(&'a str),
    Concat { head: &'a str, tail: &'a [&'a str] },
    Alternation { head: &'a str, tail: &'a [&'a str] },
}

struct Visitor;

impl Visitor {
    fn pop(&self, induct: Frame) -> Option<Frame> {
        match induct {
            Frame::Repetition(_) => None,
            Frame::Group(_) => None,
            Frame::Concat { tail, .. } => {
                if tail.is_empty() {
                    None
                } else {
                    Some(Frame::Concat {
                        head: &tail[0],
                        tail: &tail[1..],
                    })
                }
            }
            Frame::Alternation { tail, .. } => {
                if tail.is_empty() {
                    None
                } else {
                    Some(Frame::Alternation {
                        head: &tail[0],
                        tail: &tail[1..],
                    })
                }
            }
        }
    }
}

#[test]
fn test_pop_concat_non_empty_tail() {
    let visitor = Visitor;
    let induct = Frame::Concat {
        head: "a",
        tail: &["b", "c"],
    };
    
    let result = visitor.pop(induct);
    
    assert!(result.is_some());
    match result.unwrap() {
        Frame::Concat { head, tail } => {
            assert_eq!(head, "b");
            assert_eq!(tail.len(), 1);
            assert_eq!(tail[0], "c");
        },
        _ => panic!("Expected Frame::Concat"),
    }
}

#[test]
fn test_pop_concat_empty_tail() {
    let visitor = Visitor;
    let induct = Frame::Concat {
        head: "a",
        tail: &[],
    };
    
    let result = visitor.pop(induct);
    
    assert!(result.is_none());
}

#[test]
fn test_pop_alternation_non_empty_tail() {
    let visitor = Visitor;
    let induct = Frame::Alternation {
        head: "x",
        tail: &["y", "z"],
    };
    
    let result = visitor.pop(induct);
    
    assert!(result.is_some());
    match result.unwrap() {
        Frame::Alternation { head, tail } => {
            assert_eq!(head, "y");
            assert_eq!(tail.len(), 1);
            assert_eq!(tail[0], "z");
        },
        _ => panic!("Expected Frame::Alternation"),
    }
}

#[test]
fn test_pop_alternation_empty_tail() {
    let visitor = Visitor;
    let induct = Frame::Alternation {
        head: "x",
        tail: &[],
    };
    
    let result = visitor.pop(induct);
    
    assert!(result.is_none());
}

#[test]
fn test_pop_repetition() {
    let visitor = Visitor;
    let induct = Frame::Repetition("repeat");
    
    let result = visitor.pop(induct);
    
    assert!(result.is_none());
}

#[test]
fn test_pop_group() {
    let visitor = Visitor;
    let induct = Frame::Group("group");
    
    let result = visitor.pop(induct);
    
    assert!(result.is_none());
}

