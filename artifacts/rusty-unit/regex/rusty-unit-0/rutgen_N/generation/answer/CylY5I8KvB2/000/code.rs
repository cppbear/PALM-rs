// Answer 0

#[test]
fn test_child_repetition() {
    struct Repetition {
        hir: Hir,
    }

    struct Frame {
        kind: FrameKind,
    }

    enum FrameKind {
        Repetition(Repetition),
        Group(Group),
        Concat { head: &'static Hir },
        Alternation { head: &'static Hir },
    }

    struct Group {
        hir: Hir,
    }

    struct Hir;

    impl Frame {
        fn child(&self) -> &'_ Hir {
            match &self.kind {
                FrameKind::Repetition(rep) => &rep.hir,
                FrameKind::Group(group) => &group.hir,
                FrameKind::Concat { head, .. } => head,
                FrameKind::Alternation { head, .. } => head,
            }
        }
    }

    let hir = Hir;
    let rep = Repetition { hir: hir };
    let frame = Frame { kind: FrameKind::Repetition(rep) };
    assert_eq!(frame.child(), &hir);
}

#[test]
fn test_child_group() {
    struct Repetition {
        hir: Hir,
    }

    struct Frame {
        kind: FrameKind,
    }

    enum FrameKind {
        Repetition(Repetition),
        Group(Group),
        Concat { head: &'static Hir },
        Alternation { head: &'static Hir },
    }

    struct Group {
        hir: Hir,
    }

    struct Hir;

    impl Frame {
        fn child(&self) -> &'_ Hir {
            match &self.kind {
                FrameKind::Repetition(rep) => &rep.hir,
                FrameKind::Group(group) => &group.hir,
                FrameKind::Concat { head, .. } => head,
                FrameKind::Alternation { head, .. } => head,
            }
        }
    }

    let hir = Hir;
    let group = Group { hir: hir };
    let frame = Frame { kind: FrameKind::Group(group) };
    assert_eq!(frame.child(), &hir);
}

#[test]
fn test_child_concat() {
    struct Repetition {
        hir: Hir,
    }

    struct Frame {
        kind: FrameKind,
    }

    enum FrameKind {
        Repetition(Repetition),
        Group(Group),
        Concat { head: &'static Hir },
        Alternation { head: &'static Hir },
    }

    struct Group {
        hir: Hir,
    }

    struct Hir;

    impl Frame {
        fn child(&self) -> &'_ Hir {
            match &self.kind {
                FrameKind::Repetition(rep) => &rep.hir,
                FrameKind::Group(group) => &group.hir,
                FrameKind::Concat { head, .. } => head,
                FrameKind::Alternation { head, .. } => head,
            }
        }
    }

    let hir = Hir;
    let frame = Frame { kind: FrameKind::Concat { head: &hir } };
    assert_eq!(frame.child(), &hir);
}

#[test]
fn test_child_alternation() {
    struct Repetition {
        hir: Hir,
    }

    struct Frame {
        kind: FrameKind,
    }

    enum FrameKind {
        Repetition(Repetition),
        Group(Group),
        Concat { head: &'static Hir },
        Alternation { head: &'static Hir },
    }

    struct Group {
        hir: Hir,
    }

    struct Hir;

    impl Frame {
        fn child(&self) -> &'_ Hir {
            match &self.kind {
                FrameKind::Repetition(rep) => &rep.hir,
                FrameKind::Group(group) => &group.hir,
                FrameKind::Concat { head, .. } => head,
                FrameKind::Alternation { head, .. } => head,
            }
        }
    }

    let hir = Hir;
    let frame = Frame { kind: FrameKind::Alternation { head: &hir } };
    assert_eq!(frame.child(), &hir);
}

