// Answer 0

#[test]
fn test_pop_empty_concat() {
    struct TestFrame<'a>(&'a [u8]);

    enum Frame<'a> {
        Repetition(u8),
        Group(u8),
        Concat { head: &'a u8, tail: &'a [u8] },
        Alternation { head: &'a u8, tail: &'a [u8] },
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

    let empty_tail: &[u8] = &[];
    let concat_frame = Frame::Concat { head: &0, tail: empty_tail };
    let test_frame = TestFrame(&[]);

    let result = test_frame.pop(concat_frame);
    assert!(result.is_none());
}

