// Answer 0

#[test]
fn test_alternation_empty() {
    use regex_syntax::hir::{Hir, HirKind, HirInfo};

    // Test with an empty vector, expecting Hir::empty() to be returned
    let exprs: Vec<Hir> = vec![];
    let result = alternation(exprs);
    assert_eq!(result, Hir::empty());
}

#[test]
fn test_alternation_single() {
    use regex_syntax::hir::{Hir, HirKind, HirInfo};

    // Test with a single expression, expecting that expression to be returned
    let exprs: Vec<Hir> = vec![Hir { kind: HirKind::Char('a'), info: HirInfo::new() }];
    let result = alternation(exprs);
    assert_eq!(result, Hir { kind: HirKind::Char('a'), info: HirInfo::new() });
}

#[test]
fn test_alternation_multiple() {
    use regex_syntax::hir::{Hir, HirKind, HirInfo};

    // Test with multiple expressions to validate the alternation behavior
    let expr1 = Hir { kind: HirKind::Char('b'), info: HirInfo::new() };
    let expr2 = Hir { kind: HirKind::Char('c'), info: HirInfo::new() };
    let exprs: Vec<Hir> = vec![expr1, expr2];
    
    let result = alternation(exprs);
    
    assert_eq!(result.kind, HirKind::Alternation(vec![
        Hir { kind: HirKind::Char('b'), info: HirInfo::new() },
        Hir { kind: HirKind::Char('c'), info: HirInfo::new() },
    ]));
}

