// Answer 0

#[test]
fn test_prefixes_zero_or_more_literals_empty() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition { kind: RepetitionKind::ZeroOrMore, greedy: true, hir: Box::new(Hir::literal(Literal::Byte(0))) });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_zero_or_more_literals_single() {
    let mut lits = Literals::empty();
    lits.add(Literal::new(vec![b'a']));
    let expr = Hir::repetition(Repetition { kind: RepetitionKind::ZeroOrMore, greedy: true, hir: Box::new(Hir::literal(Literal::Byte(0))) });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_zero_or_more_literals_multiple() {
    let mut lits = Literals::empty();
    for i in 0..5 {
        lits.add(Literal::new(vec![b'a'; i * 10]));  // Add Literals of varying lengths (0 to 40 bytes)
    }
    let expr = Hir::repetition(Repetition { kind: RepetitionKind::ZeroOrMore, greedy: true, hir: Box::new(Hir::literal(Literal::Byte(0))) });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_zero_or_more_literals_maximum() {
    let mut lits = Literals::empty();
    for i in 0..10 {
        let byte_vec = vec![b'a'; i * 10];  // Literals of varying lengths, creating 10 Literals
        lits.add(Literal::new(byte_vec));
    }
    let expr = Hir::repetition(Repetition { kind: RepetitionKind::ZeroOrMore, greedy: true, hir: Box::new(Hir::literal(Literal::Byte(0))) });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_zero_or_more_literals_cut() {
    let mut lits = Literals::empty();
    lits.add(Literal::new(vec![b'a', b'b', b'c', b'd', b'e', b'f', b'g']));
    let expr = Hir::repetition(Repetition { kind: RepetitionKind::ZeroOrMore, greedy: false, hir: Box::new(Hir::literal(Literal::Byte(0))) });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_zero_or_more_literals_with_cut() {
    let mut lits = Literals::empty();
    lits.add(Literal::new(vec![b'a']));    // Add a literal that does not cut
    lits.add(Literal::new(vec![b'b', b'c'])); // Add a cut literal
    let expr = Hir::repetition(Repetition { kind: RepetitionKind::ZeroOrMore, greedy: true, hir: Box::new(Hir::literal(Literal::Byte(0))) });
    prefixes(&expr, &mut lits);
}

