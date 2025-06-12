fn child(&self) -> &'a Hir {
        match *self {
            Frame::Repetition(rep) => &rep.hir,
            Frame::Group(group) => &group.hir,
            Frame::Concat { head, .. } => head,
            Frame::Alternation { head, .. } => head,
        }
    }