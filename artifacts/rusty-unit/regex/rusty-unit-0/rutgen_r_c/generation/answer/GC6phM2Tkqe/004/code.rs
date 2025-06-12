// Answer 0

#[test]
fn test_induct_concat_empty() {
    use hir::{HirKind, Hir};
    
    let empty_hir = Hir::concat(Vec::new());
    let mut visitor = HeapVisitor::new();

    let result = visitor.induct(&empty_hir);
    assert_eq!(result, None);
}

#[test]
fn test_induct_concat_non_empty() {
    use hir::{HirKind, Hir};

    let non_empty_hir = Hir::concat(vec![
        Hir::literal(Literal::new('a')),
        Hir::literal(Literal::new('b')),
    ]);
    let mut visitor = HeapVisitor::new();

    let result = visitor.induct(&non_empty_hir);
    match result {
        Some(Frame::Concat { head, tail }) => {
            assert_eq!(head.kind(), &HirKind::Literal(Literal::new('a')));
            assert_eq!(tail.len(), 1);
            assert_eq!(tail[0].kind(), &HirKind::Literal(Literal::new('b')));
        },
        _ => panic!("Expected a non-empty concat frame"),
    }
}

#[test]
fn test_induct_repetition() {
    use hir::{HirKind, Hir, Repetition};

    let repetition_hir = Hir::repetition(Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::new('a'))),
    });
    let mut visitor = HeapVisitor::new();

    let result = visitor.induct(&repetition_hir);
    match result {
        Some(Frame::Repetition(ref rep)) => {
            assert!(rep.greedy);
        },
        _ => panic!("Expected a repetition frame"),
    }
}

#[test]
fn test_induct_group() {
    use hir::{HirKind, Hir, Group};

    let group_hir = Hir::group(Group {
        kind: GroupKind::Capturing(0),
        hir: Box::new(Hir::literal(Literal::new('b'))),
    });
    let mut visitor = HeapVisitor::new();

    let result = visitor.induct(&group_hir);
    match result {
        Some(Frame::Group(ref grp)) => {
            assert_eq!(grp.kind, GroupKind::Capturing(0));
        },
        _ => panic!("Expected a group frame"),
    }
}

#[test]
fn test_induct_alternation_empty() {
    use hir::{HirKind, Hir};

    let empty_alternation = Hir::alternation(Vec::new());
    let mut visitor = HeapVisitor::new();

    let result = visitor.induct(&empty_alternation);
    assert_eq!(result, None);
}

