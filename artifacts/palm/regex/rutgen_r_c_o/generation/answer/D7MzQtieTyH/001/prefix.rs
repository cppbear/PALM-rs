// Answer 0

#[test]
fn test_union_suffixes_empty_lits() {
    let mut literals = Literals::empty();
    let hir = Hir { 
        kind: HirKind::Literal(hir::Literal::Unicode('a')), 
        info: HirInfo::default() 
    };
    literals.union_suffixes(&hir);
}

#[test]
fn test_union_suffixes_non_empty_lits_with_size_limit() {
    let mut literals = Literals { 
        lits: vec![Literal::Unicode('a')], 
        limit_size: 10, 
        limit_class: 10 
    };
    let hir = Hir { 
        kind: HirKind::Literal(hir::Literal::Byte(97)), 
        info: HirInfo::default() 
    };
    literals.union_suffixes(&hir);
}

#[test]
fn test_union_suffixes_with_bounded_limits() {
    let mut literals = Literals { 
        lits: vec![], 
        limit_size: 50, 
        limit_class: 20 
    };
    let hir = Hir { 
        kind: HirKind::Class(hir::Class::Unicode(hir::ClassUnicode::default())), 
        info: HirInfo::default() 
    };
    literals.union_suffixes(&hir);
}

#[test]
fn test_union_suffixes_large_input() {
    let mut literals = Literals { 
        lits: vec![], 
        limit_size: 100, 
        limit_class: 100 
    };
    let hir = Hir { 
        kind: HirKind::Group { hir: Box::new(hir::Hir::default()) }, 
        info: HirInfo::default() 
    };
    literals.union_suffixes(&hir);
}

#[test]
#[should_panic]
fn test_union_suffixes_triggering_panic() {
    let mut literals = Literals { 
        lits: vec![], 
        limit_size: 5, 
        limit_class: 1 
    };
    let hir = Hir { 
        kind: HirKind::Class(hir::Class::Bytes(hir::ClassBytes::default())), 
        info: HirInfo::default() 
    };
    literals.union_suffixes(&hir);
}

