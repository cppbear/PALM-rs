// Answer 0

#[test]
fn test_pop_with_repetition() {
    struct TestFrame<'a>(&'a str);

    enum Frame<'a> {
        Repetition(&'a TestFrame<'a>),
        Group(&'a TestFrame<'a>),
        Concat { head: &'a TestFrame<'a>, tail: &'a [&'a TestFrame<'a>] },
        Alternation { head: &'a TestFrame<'a>, tail: &'a [&'a TestFrame<'a>] },
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

    let frame = TestFrame("test");
    let induct = Frame::Repetition(&frame);
    let result = frame.pop(induct);
    assert_eq!(result, None);
}

