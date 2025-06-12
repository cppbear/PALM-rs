// Answer 0

#[test]
fn test_child_repetition() {
    struct Repetition {
        hir: Hir,
    }

    struct Frame {
        reps: Repetition,
    }

    struct Hir;

    impl Frame {
        fn child(&self) -> &Hir {
            match *self {
                Frame::Repetition(ref rep) => &rep.hir,
                Frame::Group(ref group) => &group.hir,
                Frame::Concat { head, .. } => head,
                Frame::Alternation { head, .. } => head,
            }
        }
    }

    let hir_instance = Hir;
    let repetition_instance = Repetition { hir: hir_instance };
    let frame_instance = Frame { reps: repetition_instance };

    assert_eq!(frame_instance.child(), &hir_instance);
}

#[test]
fn test_child_group() {
    struct Group {
        hir: Hir,
    }

    struct Frame {
        group: Group,
    }

    struct Hir;

    impl Frame {
        fn child(&self) -> &Hir {
            match *self {
                Frame::Repetition(ref rep) => &rep.hir,
                Frame::Group(ref group) => &group.hir,
                Frame::Concat { head, .. } => head,
                Frame::Alternation { head, .. } => head,
            }
        }
    }

    let hir_instance = Hir;
    let group_instance = Group { hir: hir_instance };
    let frame_instance = Frame { group: group_instance };

    assert_eq!(frame_instance.child(), &hir_instance);
}

#[test]
fn test_child_concat() {
    struct Hir;

    struct Frame {
        head: &'static Hir,
    }

    impl Frame {
        fn child(&self) -> &Hir {
            match *self {
                Frame::Repetition(ref rep) => &rep.hir,
                Frame::Group(ref group) => &group.hir,
                Frame::Concat { head, .. } => head,
                Frame::Alternation { head, .. } => head,
            }
        }
    }

    let hir_instance = Hir;
    let frame_instance = Frame { head: &hir_instance };

    assert_eq!(frame_instance.child(), &hir_instance);
}

#[test]
fn test_child_alternation() {
    struct Hir;

    struct Frame {
        head: &'static Hir,
    }

    impl Frame {
        fn child(&self) -> &Hir {
            match *self {
                Frame::Repetition(ref rep) => &rep.hir,
                Frame::Group(ref group) => &group.hir,
                Frame::Concat { head, .. } => head,
                Frame::Alternation { head, .. } => head,
            }
        }
    }

    let hir_instance = Hir;
    let frame_instance = Frame { head: &hir_instance };

    assert_eq!(frame_instance.child(), &hir_instance);
}

