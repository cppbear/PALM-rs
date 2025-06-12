// Answer 0

#[test]
fn test_child_group() {
    struct Group {
        hir: Hir,
    }

    struct Hir {
        value: String,
    }

    enum Frame<'a> {
        Repetition(&'a Repetition),
        Group(&'a Group),
        Concat { head: &'a Hir, tail: &'a [Hir] },
        Alternation { head: &'a Hir, tail: &'a [Hir] },
    }

    struct Repetition {
        hir: Hir,
    }

    let child_hir = Hir { value: String::from("child") };
    let group = Group { hir: child_hir };
    let frame = Frame::Group(&group);

    match frame {
        Frame::Group(ref g) => {
            let result = g.hir.value.clone();
            assert_eq!(result, "child");
        },
        _ => panic!("Expected Frame::Group"),
    }
}

#[test]
fn test_child_repetition() {
    struct Repetition {
        hir: Hir,
    }

    struct Hir {
        value: String,
    }

    enum Frame<'a> {
        Repetition(&'a Repetition),
        Group(&'a Group),
        Concat { head: &'a Hir, tail: &'a [Hir] },
        Alternation { head: &'a Hir, tail: &'a [Hir] },
    }

    struct Group {
        hir: Hir,
    }

    let child_hir = Hir { value: String::from("repetition") };
    let repetition = Repetition { hir: child_hir };
    let frame = Frame::Repetition(&repetition);

    match frame {
        Frame::Repetition(ref rep) => {
            let result = rep.hir.value.clone();
            assert_eq!(result, "repetition");
        },
        _ => panic!("Expected Frame::Repetition"),
    }
}

#[test]
fn test_child_concat() {
    struct Hir {
        value: String,
    }

    enum Frame<'a> {
        Repetition(&'a Repetition),
        Group(&'a Group),
        Concat { head: &'a Hir, tail: &'a [Hir] },
        Alternation { head: &'a Hir, tail: &'a [Hir] },
    }

    struct Repetition {
        hir: Hir,
    }

    struct Group {
        hir: Hir,
    }

    let head_hir = Hir { value: String::from("concat_head") };
    let frame = Frame::Concat { head: &head_hir, tail: &[] };

    match frame {
        Frame::Concat { head, .. } => {
            let result = head.value.clone();
            assert_eq!(result, "concat_head");
        },
        _ => panic!("Expected Frame::Concat"),
    }
}

#[test]
fn test_child_alternation() {
    struct Hir {
        value: String,
    }

    enum Frame<'a> {
        Repetition(&'a Repetition),
        Group(&'a Group),
        Concat { head: &'a Hir, tail: &'a [Hir] },
        Alternation { head: &'a Hir, tail: &'a [Hir] },
    }

    struct Repetition {
        hir: Hir,
    }

    struct Group {
        hir: Hir,
    }

    let head_hir = Hir { value: String::from("alternation_head") };
    let frame = Frame::Alternation { head: &head_hir, tail: &[] };

    match frame {
        Frame::Alternation { head, .. } => {
            let result = head.value.clone();
            assert_eq!(result, "alternation_head");
        },
        _ => panic!("Expected Frame::Alternation"),
    }
}

