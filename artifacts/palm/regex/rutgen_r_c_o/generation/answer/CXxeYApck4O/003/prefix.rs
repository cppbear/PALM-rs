// Answer 0

#[test]
fn test_prefixes_empty_concat() {
    let expr = Hir::concat(vec![]);
    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_single_literal_concat() {
    let lit = Hir::literal(Literal::Unicode('a'));
    let expr = Hir::concat(vec![lit]);
    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_empty_literal() {
    let expr = Hir::literal(Literal::empty());
    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_empty_group() {
    let group = Hir::group(Hir::empty());
    let expr = Hir::concat(vec![group]);
    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_empty_repetition() {
    let repetition = Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    let expr = Hir::repetition(repetition);
    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);
}

