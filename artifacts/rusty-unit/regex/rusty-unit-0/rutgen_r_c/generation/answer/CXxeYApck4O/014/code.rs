// Answer 0

#[test]
fn test_prefixes_with_empty_group() {
    use hir::{self, HirKind, Group};

    let expr = Hir {
        kind: HirKind::Group(Group { hir: Box::new(Hir::empty()) }),
        info: HirInfo::default(),
    };

    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);

    assert!(lits.is_empty());
}

#[test]
fn test_prefixes_with_unambiguous_group() {
    use hir::{self, HirKind, Group, Literal};

    let expr = Hir {
        kind: HirKind::Group(Group { 
            hir: Box::new(Hir::literal(Literal::Unicode('a'))), 
        }),
        info: HirInfo::default(),
    };

    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);

    assert_eq!(lits.limit_class(), 0);
    assert_eq!(lits.limit_size(), 0);
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_with_concatenated_groups() {
    use hir::{self, HirKind, Group, Literal};

    let expr = Hir {
        kind: HirKind::Concat(vec![
            Hir::literal(Literal::Unicode('a')),
            Hir::literal(Literal::Byte(b'b')),
            Hir::group(Group {
                hir: Box::new(Hir::literal(Literal::Unicode('c'))),
            }),
        ]),
        info: HirInfo::default(),
    };

    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);

    assert!(!lits.is_empty());
    assert_eq!(lits.limit_class(), 0);
}

#[test]
fn test_prefixes_with_repetition_in_group() {
    use hir::{self, HirKind, Group, Literal, Repetition, RepetitionKind};

    let expr = Hir {
        kind: HirKind::Group(Group {
            hir: Box::new(Hir::repetition(Repetition {
                kind: RepetitionKind::ZeroOrMore,
                greedy: true,
                hir: Box::new(Hir::literal(Literal::Unicode('x'))),
            })),
        }),
        info: HirInfo::default(),
    };

    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);

    assert!(lits.any_complete());
}

#[test]
fn test_prefixes_with_alternation_in_group() {
    use hir::{self, HirKind, Group, Literal, Alternation};

    let expr = Hir {
        kind: HirKind::Group(Group {
            hir: Box::new(Hir::alternation(vec![
                Hir::literal(Literal::Unicode('x')),
                Hir::literal(Literal::Unicode('y')),
            ])),
        }),
        info: HirInfo::default(),
    };

    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);

    assert!(!lits.is_empty());
    assert_eq!(lits.limit_class(), 0);
}

#[test]
fn test_prefixes_with_class_in_group() {
    use hir::{self, HirKind, Group, Class, ClassUnicode, ClassBytes};

    let expr = Hir {
        kind: HirKind::Group(Group {
            hir: Box::new(Hir::class(Class::Unicode(ClassUnicode {
                set: IntervalSet::new(vec![0x61..=0x7A]), // 'a' to 'z'
            }))),
        }),
        info: HirInfo::default(),
    };

    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);

    assert!(lits.any_complete());
}

