// Answer 0

#[test]
fn test_repeat_zero_or_one_literals_non_empty() {
    let limit_size = 512;
    let limit_class = 5;
    
    let mut lits = Literals {
        lits: vec![Literal::new(vec![1, 2, 3])],
        limit_size,
        limit_class,
    };
    
    let expr = Hir {
        kind: HirKind::SomeKind,
        info: HirInfo::default(),
    };

    repeat_zero_or_one_literals(&expr, &mut lits, |e, lits3| {
        lits3.add(Literal::new(vec![4, 5, 6]));
    });
}

#[test]
fn test_repeat_zero_or_one_literals_cross_product() {
    let limit_size = 256;
    let limit_class = 3;

    let mut lits = Literals {
        lits: vec![Literal::new(vec![7, 8, 9]), Literal::new(vec![10, 11])],
        limit_size,
        limit_class,
    };

    let expr = Hir {
        kind: HirKind::AnotherKind,
        info: HirInfo::default(),
    };

    repeat_zero_or_one_literals(&expr, &mut lits, |e, lits3| {
        lits3.add(Literal::new(vec![12, 13]));
    });
}

#[test]
fn test_repeat_zero_or_one_literals_union() {
    let limit_size = 1024;
    let limit_class = 10;

    let mut lits = Literals {
        lits: vec![Literal::new(vec![14, 15])],
        limit_size,
        limit_class,
    };

    let expr = Hir {
        kind: HirKind::DifferentKind,
        info: HirInfo::default(),
    };

    repeat_zero_or_one_literals(&expr, &mut lits, |e, lits3| {
        lits3.add(Literal::new(vec![16]));
        lits3.add(Literal::new(vec![17, 18]));
    });
}

