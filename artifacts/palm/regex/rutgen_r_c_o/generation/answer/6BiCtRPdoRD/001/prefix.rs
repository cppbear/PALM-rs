// Answer 0

#[test]
fn test_suffixes_empty_hir() {
    let expr = Hir { kind: HirKind::Empty, info: Default::default() };
    suffixes(&expr);
}

#[test]
fn test_suffixes_hir_with_characters() {
    let expr = Hir {
        kind: HirKind::Concat(vec![Literal::Unicode('a'), Literal::Byte(0x62)]),
        info: Default::default(),
    };
    suffixes(&expr);
}

#[test]
fn test_suffixes_hir_exceeding_limit_size() {
    let expr = Hir {
        kind: HirKind::Concat(vec![Literal::Unicode('x'); 300]),
        info: Default::default(),
    };
    suffixes(&expr);
}

#[test]
fn test_suffixes_hir_exceeding_limit_class() {
    let expr = Hir {
        kind: HirKind::Union(vec![
            Literal::Unicode('y'),
            Literal::Unicode('z'),
            Literal::Unicode('w'),
            Literal::Unicode('v'),
            Literal::Unicode('u'),
            Literal::Unicode('t'),
            Literal::Unicode('s'),
            Literal::Unicode('r'),
            Literal::Unicode('q'),
            Literal::Unicode('p'),
            Literal::Unicode('o'),
            Literal::Unicode('n'),
        ]),
        info: Default::default(),
    };
    suffixes(&expr);
}

#[test]
fn test_suffixes_hir_with_mixed_character_classes() {
    let expr = Hir {
        kind: HirKind::Concat(vec![
            Literal::Unicode('a'),
            Literal::Byte(0x62),
            Literal::Unicode('c'),
        ]),
        info: Default::default(),
    };
    suffixes(&expr);
}

#[test]
fn test_suffixes_hir_with_unicode_and_arbitrary_bytes() {
    let expr = Hir {
        kind: HirKind::Union(vec![
            Literal::Unicode('α'),
            Literal::Byte(0x61),
        ]),
        info: Default::default(),
    };
    suffixes(&expr);
}

