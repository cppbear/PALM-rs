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