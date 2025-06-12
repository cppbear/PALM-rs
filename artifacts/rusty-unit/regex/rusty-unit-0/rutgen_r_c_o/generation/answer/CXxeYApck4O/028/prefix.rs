// Answer 0

#[test]
fn test_prefixes_literal_byte() {
    let mut lits = Literals::empty();
    let byte_literal = Literal::Byte(128);
    let hir = Hir::literal(byte_literal);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_literal_unicode() {
    let mut lits = Literals::empty();
    let unicode_literal = Literal::Unicode('A');
    let hir = Hir::literal(unicode_literal);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_empty_concat() {
    let mut lits = Literals::empty();
    let hir = Hir::concat(vec![]);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_single_literal_unicode() {
    let mut lits = Literals::empty();
    let unicode_literal = Literal::Unicode('b');
    let hir = Hir::literal(unicode_literal);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_single_literal_byte() {
    let mut lits = Literals::empty();
    let byte_literal = Literal::Byte(255);
    let hir = Hir::literal(byte_literal);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_literal_unicode_composite() {
    let mut lits = Literals::empty();
    let unicode_literal1 = Literal::Unicode('C');
    let unicode_literal2 = Literal::Unicode('D');
    let hir = Hir::concat(vec![Hir::literal(unicode_literal1), Hir::literal(unicode_literal2)]);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_repetition_zero_or_one() {
    let mut lits = Literals::empty();
    let unicode_literal = Literal::Unicode('E');
    let repetition = Repetition {
        kind: RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(Hir::literal(unicode_literal)),
    };
    let hir = Hir::repetition(repetition);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_repetition_one_or_more() {
    let mut lits = Literals::empty();
    let unicode_literal = Literal::Unicode('F');
    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: false,
        hir: Box::new(Hir::literal(unicode_literal)),
    };
    let hir = Hir::repetition(repetition);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_unicode_range() {
    let mut lits = Literals::empty();
    let unicode_class = Class::Unicode(ClassUnicode { set: IntervalSet::new() });
    let hir = Hir::class(unicode_class);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_byte_range() {
    let mut lits = Literals::empty();
    let byte_class = Class::Bytes(ClassBytes { set: IntervalSet::new() });
    let hir = Hir::class(byte_class);
    prefixes(&hir, &mut lits);
}

