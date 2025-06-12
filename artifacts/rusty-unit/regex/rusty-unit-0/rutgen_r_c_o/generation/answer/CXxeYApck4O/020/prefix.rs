// Answer 0

#[test]
fn test_prefixes_repetition_zero_or_one() {
    let mut lits = Literals::empty();
    
    let lit = Literal::new(vec![b'a']);
    let repetition = Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    };
    
    let expr = Hir::repetition(repetition);
    
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_zero_or_one_empty() {
    let mut lits = Literals::empty();
    
    let lit = Literal::empty();
    let repetition = Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    };
    
    let expr = Hir::repetition(repetition);
    
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_zero_or_one_multiple_literals() {
    let mut lits = Literals::empty();
    
    let lit1 = Literal::new(vec![b'b']);
    let lit2 = Literal::new(vec![b'c']);
    
    let repetition = Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(Hir::alternation(vec![
            Hir::literal(lit1),
            Hir::literal(lit2),
        ])),
    };
    
    let expr = Hir::repetition(repetition);
    
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_zero_or_one_cut() {
    let mut lits = Literals::empty();
    
    let lit = Literal::new(vec![b'z']);
    let repetition = Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: false,
        hir: Box::new(Hir::literal(lit)),
    };
    
    let expr = Hir::repetition(repetition);
    
    prefixes(&expr, &mut lits);
}

