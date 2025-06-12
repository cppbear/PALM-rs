// Answer 0

#[test]
fn test_repetition_one_or_more_greedy() {
    let literal_hir = Hir::literal(Literal::from('a'));
    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(literal_hir),
    };
    let _ = repetition(repetition);
}

#[test]
fn test_repetition_one_or_more_non_greedy() {
    let class_hir = Hir::class(Class::from(vec!['a', 'b', 'c']));
    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: false,
        hir: Box::new(class_hir),
    };
    let _ = repetition(repetition);
}

#[test]
fn test_repetition_zero_or_more_greedy_with_complex_hir() {
    let concat_hir = Hir::concat(vec![
        Hir::literal(Literal::from('x')),
        Hir::literal(Literal::from('y')),
    ]);
    let repetition = Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(concat_hir),
    };
    let _ = repetition(repetition);
}

#[test]
fn test_repetition_zero_or_more_non_greedy_with_anchor() {
    let anchor_hir = Hir::anchor(Anchor::start_of_line());
    let repetition = Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: false,
        hir: Box::new(anchor_hir),
    };
    let _ = repetition(repetition);
}

#[test]
fn test_repetition_bounded_range() {
    let word_boundary_hir = Hir::word_boundary(WordBoundary::unicode());
    let repetition = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(2, 5)),
        greedy: true,
        hir: Box::new(word_boundary_hir),
    };
    let _ = repetition(repetition);
}

