// Answer 0

#[derive(Debug)]
enum Frame<'a> {
    Repetition(&'a str),
    Group(&'a str),
    Concat { head: &'a str, tail: &'a [&'a str] },
    Alternation { head: &'a str, tail: &'a [&'a str] },
}

impl<'a> Frame<'a> {
    fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>> {
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
fn test_pop_concat_with_non_empty_tail() {
    let tail = ["b", "c"];
    let frame = Frame::Concat {
        head: "a",
        tail: &tail,
    };
    let result = frame.pop(frame);
    assert!(result.is_some());
    if let Some(Frame::Concat { head, tail }) = result {
        assert_eq!(head, "b");
        assert_eq!(tail, &["c"]);
    } else {
        panic!("Expected Some for non-empty tail");
    }
}

#[test]
fn test_pop_concat_with_empty_tail() {
    let tail: [&str; 0] = [];
    let frame = Frame::Concat {
        head: "a",
        tail: &tail,
    };
    let result = frame.pop(frame);
    assert!(result.is_none());
}

#[test]
fn test_pop_alternation_with_non_empty_tail() {
    let tail = ["b", "c"];
    let frame = Frame::Alternation {
        head: "a",
        tail: &tail,
    };
    let result = frame.pop(frame);
    assert!(result.is_some());
    if let Some(Frame::Alternation { head, tail }) = result {
        assert_eq!(head, "b");
        assert_eq!(tail, &["c"]);
    } else {
        panic!("Expected Some for non-empty tail");
    }
}

#[test]
fn test_pop_alternation_with_empty_tail() {
    let tail: [&str; 0] = [];
    let frame = Frame::Alternation {
        head: "a",
        tail: &tail,
    };
    let result = frame.pop(frame);
    assert!(result.is_none());
}

#[test]
fn test_pop_group() {
    let frame = Frame::Group("a");
    let result = frame.pop(frame);
    assert!(result.is_none());
}

#[test]
fn test_pop_repetition() {
    let frame = Frame::Repetition("a");
    let result = frame.pop(frame);
    assert!(result.is_none());
}

