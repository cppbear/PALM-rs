// Answer 0

#[test]
fn test_prefixes_repetition_range_at_least() {
    let mut lits = Literals::empty();
    let bytes = vec![b'a', b'b', b'c'];
    let lit = Literal::new(bytes);
    let rep = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(3)),
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    };
    let expr = Hir::repetition(rep);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_range_at_least_large() {
    let mut lits = Literals::empty();
    let bytes = vec![b'x', b'y', b'z'];
    let lit = Literal::new(bytes);
    let rep = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(10)),
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    };
    let expr = Hir::repetition(rep);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_range_at_least_edge_case() {
    let mut lits = Literals::empty();
    let bytes = vec![b'1'];
    let lit = Literal::new(bytes);
    let rep = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(1)),
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    };
    let expr = Hir::repetition(rep);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_range_at_least_multiple_repetitions() {
    let mut lits = Literals::empty();
    let bytes = vec![b'a', b'b'];
    let lit = Literal::new(bytes);
    let rep1 = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(2)),
        greedy: true,
        hir: Box::new(Hir::literal(lit.clone())),
    };
    let rep2 = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(3)),
        greedy: false,
        hir: Box::new(Hir::literal(lit)),
    };
    let expr = Hir::concat(vec![Hir::repetition(rep1), Hir::repetition(rep2)]);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_range_at_least_minimum() {
    let mut lits = Literals::empty();
    let bytes = vec![b'c'];
    let lit = Literal::new(bytes);
    let rep = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(1)),
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    };
    let expr = Hir::repetition(rep);
    prefixes(&expr, &mut lits);
}

