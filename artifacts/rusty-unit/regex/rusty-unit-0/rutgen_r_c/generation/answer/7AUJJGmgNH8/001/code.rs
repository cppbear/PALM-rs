// Answer 0

#[test]
fn test_hir_kind_empty() {
    let hir = Hir {
        kind: HirKind::Empty,
        info: HirInfo { bools: 0 },
    };
    assert_eq!(hir.kind(), &HirKind::Empty);
}

#[test]
fn test_hir_kind_literal() {
    let literal = Literal::from_char('a').unwrap();
    let hir = Hir {
        kind: HirKind::Literal(literal),
        info: HirInfo { bools: 0 },
    };
    assert_eq!(hir.kind(), &HirKind::Literal(literal));
}

#[test]
fn test_hir_kind_class() {
    let class = Class::new(vec!['a', 'b', 'c'].into_iter().collect()).unwrap();
    let hir = Hir {
        kind: HirKind::Class(class),
        info: HirInfo { bools: 0 },
    };
    assert_eq!(hir.kind(), &HirKind::Class(class));
}

#[test]
fn test_hir_kind_anchor() {
    let anchor = Anchor;
    let hir = Hir {
        kind: HirKind::Anchor(anchor),
        info: HirInfo { bools: 0 },
    };
    assert_eq!(hir.kind(), &HirKind::Anchor(anchor));
}

#[test]
fn test_hir_kind_word_boundary() {
    let word_boundary = WordBoundary;
    let hir = Hir {
        kind: HirKind::WordBoundary(word_boundary),
        info: HirInfo { bools: 0 },
    };
    assert_eq!(hir.kind(), &HirKind::WordBoundary(word_boundary));
}

#[test]
fn test_hir_kind_repetition() {
    let rep = Repetition::new(1, Some(3));
    let hir = Hir {
        kind: HirKind::Repetition(rep),
        info: HirInfo { bools: 0 },
    };
    assert_eq!(hir.kind(), &HirKind::Repetition(rep));
}

#[test]
fn test_hir_kind_group() {
    let group = Group::new(vec![]);
    let hir = Hir {
        kind: HirKind::Group(group),
        info: HirInfo { bools: 0 },
    };
    assert_eq!(hir.kind(), &HirKind::Group(group));
}

#[test]
fn test_hir_kind_concat() {
    let concat = HirKind::Concat(vec![
        Hir::empty(),
        Hir::literal(Literal::from_char('b').unwrap()),
    ]);
    let hir = Hir {
        kind: concat,
        info: HirInfo { bools: 0 },
    };
    assert_eq!(hir.kind(), &hir.kind());
}

#[test]
fn test_hir_kind_alternation() {
    let alt = HirKind::Alternation(vec![
        Hir::empty(),
        Hir::literal(Literal::from_char('c').unwrap()),
    ]);
    let hir = Hir {
        kind: alt,
        info: HirInfo { bools: 0 },
    };
    assert_eq!(hir.kind(), &hir.kind());
}

