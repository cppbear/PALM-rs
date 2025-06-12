// Answer 0

#[test]
fn test_pop_alternation_with_non_empty_tail() {
    struct TestFrame<'a> {
        tail: &'a [&'a str],
    }

    enum Frame<'a> {
        Repetition(u32),
        Group(u32),
        Concat { head: &'a str, tail: &'a [&'a str] },
        Alternation { head: &'a str, tail: &'a [&'a str] },
    }

    impl<'a> TestFrame<'a> {
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

    let tail = &["option1", "option2", "option3"];
    let induct = Frame::Alternation { head: "", tail };
    let result = TestFrame { tail }.pop(induct);

    match result {
        Some(Frame::Alternation { head, tail }) => {
            assert_eq!(head, &"option1");
            assert_eq!(tail, &["option2", "option3"]);
        },
        _ => panic!("Expected Some(Frame::Alternation)"),
    }
}

#[test]
fn test_pop_alternation_with_single_element_tail() {
    struct TestFrame<'a> {
        tail: &'a [&'a str],
    }

    enum Frame<'a> {
        Repetition(u32),
        Group(u32),
        Concat { head: &'a str, tail: &'a [&'a str] },
        Alternation { head: &'a str, tail: &'a [&'a str] },
    }

    impl<'a> TestFrame<'a> {
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

    let tail = &["only"];
    let induct = Frame::Alternation { head: "", tail };
    let result = TestFrame { tail }.pop(induct);

    match result {
        Some(Frame::Alternation { head, tail }) => {
            assert_eq!(head, &"only");
            assert_eq!(tail.len(), 0);
        },
        _ => panic!("Expected Some(Frame::Alternation)"),
    }
}

