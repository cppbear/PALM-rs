fn induct(&mut self, hir: &'a Hir) -> Option<Frame<'a>> {
        match *hir.kind() {
            HirKind::Repetition(ref x) => Some(Frame::Repetition(x)),
            HirKind::Group(ref x) => Some(Frame::Group(x)),
            HirKind::Concat(ref x) if x.is_empty() => None,
            HirKind::Concat(ref x) => {
                Some(Frame::Concat {
                    head: &x[0],
                    tail: &x[1..],
                })
            }
            HirKind::Alternation(ref x) if x.is_empty() => None,
            HirKind::Alternation(ref x) => {
                Some(Frame::Alternation {
                    head: &x[0],
                    tail: &x[1..],
                })
            }
            _ => None,
        }
    }