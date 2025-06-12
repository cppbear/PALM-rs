// Answer 0

#[test]
fn test_suffixes_unicode_class() {
    let unicode_class = Class::Unicode(ClassUnicode {
        set: IntervalSet::new(vec![ClassUnicodeRange::new(97, 122)]) // 'a' to 'z'
    });
    let expr = Hir::class(unicode_class);
    let mut lits = Literals::empty();

    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_bytes_class() {
    let bytes_class = Class::Bytes(ClassBytes {
        set: IntervalSet::new(vec![ClassBytesRange::new(0, 255)]) // All bytes
    });
    let expr = Hir::class(bytes_class);
    let mut lits = Literals::empty();

    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_single_unicode_literal() {
    let lit = Literal::Unicode('a'); // 'a'
    let expr = Hir::literal(lit);
    let mut lits = Literals::empty();

    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_single_byte_literal() {
    let lit = Literal::Byte(97); // ASCII value for 'a'
    let expr = Hir::literal(lit);
    let mut lits = Literals::empty();

    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_empty_concat() {
    let expr = Hir::concat(vec![]);
    let mut lits = Literals::empty();

    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_single_concat() {
    let lit = Literal::Unicode('b'); // 'b'
    let expr = Hir::concat(vec![Hir::literal(lit)]);
    let mut lits = Literals::empty();

    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_zero_or_more() {
    let lit = Literal::Byte(100); // 'd'
    let repetition = Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::literal(lit)),
    };
    let expr = Hir::repetition(repetition);
    let mut lits = Literals::empty();

    suffixes(&expr, &mut lits);
}

