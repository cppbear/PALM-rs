// Answer 0

#[test]
fn test_pop_concat_non_empty_tail() {
    struct TestFrame<'a> {
        tail: &'a [&'a str],
    }

    enum Frame<'a> {
        Repetition(&'a str),
        Group(&'a str),
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

    let tail = &["a", "b", "c"];
    let induct = Frame::Concat { head: "", tail };
    
    let result = TestFrame { tail }.pop(induct);
    
    match result {
        Some(Frame::Concat { head, tail }) => {
            assert_eq!(head, &"a");
            assert_eq!(tail, &["b", "c"]);
        },
        _ => panic!("Expected Some(Frame::Concat)"),
    }
}

#[test]
fn test_pop_concat_single_element_tail() {
    struct TestFrame<'a> {
        tail: &'a [&'a str],
    }

    enum Frame<'a> {
        Repetition(&'a str),
        Group(&'a str),
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

    let tail = &["a"];
    let induct = Frame::Concat { head: "", tail };
    
    let result = TestFrame { tail }.pop(induct);
    
    match result {
        Some(Frame::Concat { head, tail }) => {
            assert_eq!(head, &"a");
            assert_eq!(tail.len(), 0);
        },
        _ => panic!("Expected Some(Frame::Concat)"),
    }
}

#[test]
fn test_pop_concat_empty_tail() {
    struct TestFrame<'a> {
        tail: &'a [&'a str],
    }

    enum Frame<'a> {
        Repetition(&'a str),
        Group(&'a str),
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

    let tail: &[&str] = &[];
    let induct = Frame::Concat { head: "", tail };
    
    let result = TestFrame { tail }.pop(induct);
    
    assert_eq!(result, None);
}

