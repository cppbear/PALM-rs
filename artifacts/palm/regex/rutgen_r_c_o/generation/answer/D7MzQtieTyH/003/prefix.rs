// Answer 0

#[test]
fn test_union_suffixes_non_empty_literals() {
    let mut literals = Literals::empty();
    literals.set_limit_size(100).set_limit_class(50);
    
    let suffix_expr = Hir { 
        kind: HirKind::Concat(vec![
            HirKind::Literal(hir::Literal::Unicode('a')),
            HirKind::Literal(hir::Literal::Byte(255))
        ]),
        info: HirInfo::default(),
    };
    
    literals.union_suffixes(&suffix_expr);
}

#[test]
fn test_union_suffixes_multiple_non_empty_literals() {
    let mut literals = Literals::empty();
    literals.set_limit_size(150).set_limit_class(30);

    let suffix_expr = Hir {
        kind: HirKind::Alternation(vec![
            HirKind::Literal(hir::Literal::Unicode('b')),
            HirKind::Literal(hir::Literal::Unicode('c')),
            HirKind::Literal(hir::Literal::Byte(100))
        ]),
        info: HirInfo::default(),
    };
    
    literals.union_suffixes(&suffix_expr);
}

#[test]
fn test_union_suffixes_with_class_and_size_limits() {
    let mut literals = Literals::empty();
    literals.set_limit_size(50).set_limit_class(20);

    let suffix_expr = Hir {
        kind: HirKind::Repetition(hir::Repetition {
            hir: Box::new(Hir {
                kind: HirKind::Literal(hir::Literal::Unicode('d')),
                info: HirInfo::default(),
            }),
            kind: hir::RepetitionKind::OneOrMore,
            greedy: true,
        }),
        info: HirInfo::default(),
    };
    
    literals.union_suffixes(&suffix_expr);
}

#[test]
fn test_union_suffixes_edge_case_empty() {
    let mut literals = Literals::empty();
    literals.set_limit_size(100).set_limit_class(50);
    
    let suffix_expr = Hir {
        kind: HirKind::Concat(vec![
            HirKind::Literal(hir::Literal::Unicode('e')),
            HirKind::Literal(hir::Literal::Byte(0))
        ]),
        info: HirInfo::default(),
    };
    
    literals.union_suffixes(&suffix_expr);
}

