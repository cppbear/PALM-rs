// Answer 0

#[test]
fn test_suffixes_repetition_at_least_zero() {
    let lit = Literal::Byte(97); // byte 'a'
    let mut expr = Hir::repetition(Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(0)),
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    });
    let mut lits = Literals::empty().set_limit_size(4096).set_limit_class(1000);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_at_least_five() {
    let lit = Literal::Unicode('b');
    let mut expr = Hir::repetition(Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(5)),
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    });
    let mut lits = Literals::empty().set_limit_size(4096).set_limit_class(1000);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_at_least_one_range() {
    let lit = Literal::Byte(120); // byte 'x'
    let mut expr = Hir::repetition(Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(1)),
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    });
    let mut lits = Literals::empty().set_limit_size(4096).set_limit_class(1000);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_at_least_ten() {
    let lit = Literal::Unicode('z');
    let mut expr = Hir::repetition(Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(10)),
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    });
    let mut lits = Literals::empty().set_limit_size(4096).set_limit_class(1000);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_at_least_twenty() {
    let lit = Literal::Byte(99); // byte 'c'
    let mut expr = Hir::repetition(Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(20)),
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    });
    let mut lits = Literals::empty().set_limit_size(4096).set_limit_class(1000);
    suffixes(&expr, &mut lits);
}

