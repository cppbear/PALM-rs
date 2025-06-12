// Answer 0

#[test]
fn test_is_always_utf8_with_minimum_bools() {
    let info = HirInfo { bools: 1 };
    let hir = Hir { kind: HirKind::Empty, info };
    hir.is_always_utf8();
}

#[test]
fn test_is_always_utf8_with_middle_bools() {
    let info = HirInfo { bools: 128 };
    let hir = Hir { kind: HirKind::Literal(Literal::from('a')), info };
    hir.is_always_utf8();
}

#[test]
fn test_is_always_utf8_with_maximum_bools() {
    let info = HirInfo { bools: 255 };
    let hir = Hir { kind: HirKind::Class(Class::default()), info };
    hir.is_always_utf8();
}

#[test]
fn test_is_always_utf8_with_no_bools() {
    let info = HirInfo { bools: 0 };
    let hir = Hir { kind: HirKind::Concat(vec![]), info };
    hir.is_always_utf8();
}

#[test]
fn test_is_always_utf8_with_varied_assertions() {
    let info1 = HirInfo { bools: 10 };
    let hir1 = Hir { kind: HirKind::WordBoundary(WordBoundary::default()), info1 };

    let info2 = HirInfo { bools: 20 };
    let hir2 = Hir { kind: HirKind::Anchor(Anchor::default()), info2 };

    hir1.is_always_utf8();
    hir2.is_always_utf8();
}

#[test]
fn test_is_always_utf8_for_repetition() {
    let info = HirInfo { bools: 64 };
    let hir = Hir { kind: HirKind::Repetition(Repetition::default()), info };
    hir.is_always_utf8();
}

#[test]
fn test_is_always_utf8_for_group() {
    let info = HirInfo { bools: 32 };
    let hir = Hir { kind: HirKind::Group(Group::default()), info };
    hir.is_always_utf8();
}

#[test]
fn test_is_always_utf8_for_empty_expression() {
    let info = HirInfo { bools: 5 };
    let hir = Hir { kind: HirKind::empty(), info };
    hir.is_always_utf8();
}

