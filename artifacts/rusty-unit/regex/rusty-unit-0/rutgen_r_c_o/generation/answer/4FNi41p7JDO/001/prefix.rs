// Answer 0

#[test]
fn test_fmt_empty_hir() {
    let h = Hir {
        kind: HirKind::Empty,
        info: HirInfo { bools: 0 },
    };
    let mut buf = String::new();
    let _ = write!(&mut buf, "{}", h);
}

#[test]
fn test_fmt_literal_hir() {
    let h = Hir {
        kind: HirKind::Literal(Literal::from('a')),
        info: HirInfo { bools: 1 },
    };
    let mut buf = String::new();
    let _ = write!(&mut buf, "{}", h);
}

#[test]
fn test_fmt_class_hir() {
    let h = Hir {
        kind: HirKind::Class(Class::new(vec!['a', 'b', 'c'])),
        info: HirInfo { bools: 15 },
    };
    let mut buf = String::new();
    let _ = write!(&mut buf, "{}", h);
}

#[test]
fn test_fmt_anchor_hir() {
    let h = Hir {
        kind: HirKind::Anchor(Anchor::new()),
        info: HirInfo { bools: 128 },
    };
    let mut buf = String::new();
    let _ = write!(&mut buf, "{}", h);
}

#[test]
fn test_fmt_word_boundary_hir() {
    let h = Hir {
        kind: HirKind::WordBoundary(WordBoundary::new()),
        info: HirInfo { bools: 255 },
    };
    let mut buf = String::new();
    let _ = write!(&mut buf, "{}", h);
}

#[test]
fn test_fmt_repetition_hir() {
    let h = Hir {
        kind: HirKind::Repetition(Repetition::new(2, 5)),
        info: HirInfo { bools: 2 },
    };
    let mut buf = String::new();
    let _ = write!(&mut buf, "{}", h);
}

#[test]
fn test_fmt_group_hir() {
    let child_hirs = vec![
        Hir {
            kind: HirKind::Literal(Literal::from('x')),
            info: HirInfo { bools: 4 },
        },
    ];
    let h = Hir {
        kind: HirKind::Group(Group::new(child_hirs)),
        info: HirInfo { bools: 0 },
    };
    let mut buf = String::new();
    let _ = write!(&mut buf, "{}", h);
}

#[test]
fn test_fmt_concat_hir() {
    let child_hirs = vec![
        Hir {
            kind: HirKind::Literal(Literal::from('x')),
            info: HirInfo { bools: 1 },
        },
        Hir {
            kind: HirKind::Literal(Literal::from('y')),
            info: HirInfo { bools: 1 },
        },
    ];
    let h = Hir {
        kind: HirKind::Concat(child_hirs),
        info: HirInfo { bools: 255 },
    };
    let mut buf = String::new();
    let _ = write!(&mut buf, "{}", h);
}

#[test]
fn test_fmt_alternation_hir() {
    let child_hirs = vec![
        Hir {
            kind: HirKind::Literal(Literal::from('x')),
            info: HirInfo { bools: 1 },
        },
        Hir {
            kind: HirKind::Literal(Literal::from('y')),
            info: HirInfo { bools: 2 },
        },
    ];
    let h = Hir {
        kind: HirKind::Alternation(child_hirs),
        info: HirInfo { bools: 15 },
    };
    let mut buf = String::new();
    let _ = write!(&mut buf, "{}", h);
}

