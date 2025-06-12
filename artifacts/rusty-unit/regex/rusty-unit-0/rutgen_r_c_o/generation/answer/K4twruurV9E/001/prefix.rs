// Answer 0

#[test]
fn test_repeat_one_or_more_literals_with_empty_hir() {
    let expr = Hir { kind: HirKind::Empty, info: HirInfo::default() };
    let mut literals = Literals::empty();
    repeat_one_or_more_literals(&expr, &mut literals, |_, _| {});
}

#[test]
fn test_repeat_one_or_more_literals_with_small_size() {
    let expr = Hir { kind: HirKind::Literal, info: HirInfo::default() };
    let mut literals = Literals { lits: vec![Literal::default()], limit_size: 10, limit_class: 5 };
    repeat_one_or_more_literals(&expr, &mut literals, |_, _| {});
}

#[test]
fn test_repeat_one_or_more_literals_with_large_size() {
    let expr = Hir { kind: HirKind::Concatenation, info: HirInfo::default() };
    let mut literals = Literals { lits: vec![Literal::default(); 10], limit_size: 100, limit_class: 50 };
    repeat_one_or_more_literals(&expr, &mut literals, |_, _| {});
}

#[test]
fn test_repeat_one_or_more_literals_with_multiple_hirs() {
    let exprs = vec![
        Hir { kind: HirKind::Literal, info: HirInfo::default() },
        Hir { kind: HirKind::Class, info: HirInfo::default() },
    ];
    let mut literals = Literals::empty();
    for expr in exprs {
        repeat_one_or_more_literals(&expr, &mut literals, |_, _| {});
    }
}

#[test]
fn test_repeat_one_or_more_literals_with_limit_class() {
    let expr = Hir { kind: HirKind::Union, info: HirInfo::default() };
    let mut literals = Literals { lits: vec![Literal::default(); 5], limit_size: 20, limit_class: 50 };
    repeat_one_or_more_literals(&expr, &mut literals, |_, _| {});
}

#[test]
fn test_repeat_one_or_more_literals_with_high_function_calls() {
    let expr = Hir { kind: HirKind::Capture, info: HirInfo::default() };
    let mut literals = Literals::empty();
    for _ in 0..5 {
        repeat_one_or_more_literals(&expr, &mut literals, |_, _| {});
    }
}

