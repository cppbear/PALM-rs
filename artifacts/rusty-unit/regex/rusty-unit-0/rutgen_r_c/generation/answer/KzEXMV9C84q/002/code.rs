// Answer 0

#[test]
fn test_repeat_zero_or_more_literals() {
    #[derive(Clone)]
    struct TestHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }
    
    let expr = TestHir { 
        kind: hir::HirKind::Literal, 
        info: hir::HirInfo::default() 
    };

    let mut lits = Literals {
        lits: vec![],
        limit_size: 100,
        limit_class: 10,
    };
    
    let lit1 = Literal { v: vec![b'a'], cut: false };
    let lit2 = Literal { v: vec![b'b'], cut: false };
    
    lits.add(lit1.clone());
    lits.add(lit2.clone());

    let mut f = |e: &Hir, lits3: &mut Literals| {
        lits3.add(lit1.clone());
        lits3.add(lit2.clone());
    };
    
    repeat_zero_or_more_literals(&expr, &mut lits, &mut f);

    assert!(!lits.is_empty());
}

