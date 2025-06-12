// Answer 0

#[test]
fn test_is_anchored_start_empty() {
    let info = HirInfo { bools: 0b00000000 };
    let hir = Hir { kind: HirKind::Empty, info };
    assert!(!hir.is_anchored_start());
}

#[test]
fn test_is_anchored_start_literal() {
    let info = HirInfo { bools: 0b00000000 }; // non-anchored
    let hir = Hir { kind: HirKind::Literal(Literal::new('a')), info };
    assert!(!hir.is_anchored_start());

    let info = HirInfo { bools: 0b00000001 }; // anchored
    let hir_anchored = Hir { kind: HirKind::Literal(Literal::new('^')), info };
    assert!(hir_anchored.is_anchored_start());
}

#[test]
fn test_is_anchored_start_class() {
    let info = HirInfo { bools: 0b00000000 }; // non-anchored
    let hir = Hir { kind: HirKind::Class(Class::new(vec!['a', 'b', 'c'])), info };
    assert!(!hir.is_anchored_start());

    let info = HirInfo { bools: 0b00000001 }; // anchored
    let hir_anchored = Hir { kind: HirKind::Class(Class::new(vec!['^', 'a', 'b'])), info };
    assert!(hir_anchored.is_anchored_start());
}

#[test]
fn test_is_anchored_start_concatenation() {
    let info = HirInfo { bools: 0b00000001 }; // anchored start
    let hir = Hir { kind: HirKind::Concat(vec![
        Hir { kind: HirKind::Literal(Literal::new('^')), info: HirInfo { bools: 0b00000001 }},
        Hir { kind: HirKind::Literal(Literal::new('a')), info: HirInfo { bools: 0b00000000 }},
    ]), info };
    assert!(hir.is_anchored_start());

    let info = HirInfo { bools: 0b00000000 }; // non-anchored start
    let hir_non_anchored = Hir { kind: HirKind::Concat(vec![
        Hir { kind: HirKind::Literal(Literal::new('a')), info: HirInfo { bools: 0b00000000 }},
        Hir { kind: HirKind::Literal(Literal::new('b')), info: HirInfo { bools: 0b00000000 }},
    ]), info };
    assert!(!hir_non_anchored.is_anchored_start());
}

#[test]
fn test_is_anchored_start_alternation() {
    let info = HirInfo { bools: 0b00000001 }; // anchored
    let hir = Hir { kind: HirKind::Alternation(vec![
        Hir { kind: HirKind::Literal(Literal::new('^')), info: HirInfo { bools: 0b00000001 }},
        Hir { kind: HirKind::Literal(Literal::new('b')), info: HirInfo { bools: 0b00000000 }},
    ]), info };
    assert!(hir.is_anchored_start());

    let info = HirInfo { bools: 0b00000000 }; // non-anchored
    let hir_non_anchored = Hir { kind: HirKind::Alternation(vec![
        Hir { kind: HirKind::Literal(Literal::new('a')), info: HirInfo { bools: 0b00000000 }},
        Hir { kind: HirKind::Literal(Literal::new('b')), info: HirInfo { bools: 0b00000000 }},
    ]), info };
    assert!(!hir_non_anchored.is_anchored_start());
}

