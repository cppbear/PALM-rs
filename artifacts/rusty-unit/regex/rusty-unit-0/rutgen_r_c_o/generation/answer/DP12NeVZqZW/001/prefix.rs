// Answer 0

#[test]
fn test_is_any_anchored_end_empty() {
    let hir = Hir::empty();
    hir.is_any_anchored_end();
}

#[test]
fn test_is_any_anchored_end_literal_no_anchor() {
    let literal = Literal::from_char('a');
    let hir = Hir::literal(literal);
    hir.is_any_anchored_end();
}

#[test]
fn test_is_any_anchored_end_literal_with_anchor() {
    let literal = Literal::from_char('$');
    let hir = Hir::literal(literal);
    hir.is_any_anchored_end();
}

#[test]
fn test_is_any_anchored_end_class_no_anchor() {
    let class = Class::new(vec!['a', 'b']);
    let hir = Hir::class(class);
    hir.is_any_anchored_end();
}

#[test]
fn test_is_any_anchored_end_class_with_anchor() {
    let class = Class::new(vec!['$', 'a', 'b']);
    let hir = Hir::class(class);
    hir.is_any_anchored_end();
}

#[test]
fn test_is_any_anchored_end_concatenation() {
    let literal_one = Literal::from_char('a');
    let literal_two = Literal::from_char('$');
    let hir = Hir::concat(vec![Hir::literal(literal_one), Hir::literal(literal_two)]);
    hir.is_any_anchored_end();
}

#[test]
fn test_is_any_anchored_end_alternation() {
    let literal_one = Literal::from_char('a');
    let literal_two = Literal::from_char('$');
    let hir = Hir::alternation(vec![Hir::literal(literal_one), Hir::literal(literal_two)]);
    hir.is_any_anchored_end();
}

#[test]
fn test_is_any_anchored_end_word_boundary() {
    let word_boundary = WordBoundary::new();
    let hir = Hir::word_boundary(word_boundary);
    hir.is_any_anchored_end();
}

#[test]
fn test_is_any_anchored_end_repetition_no_anchor() {
    let literal = Literal::from_char('a');
    let rep = Repetition::new(Hir::literal(literal), 1..5);
    let hir = Hir::repetition(rep);
    hir.is_any_anchored_end();
}

#[test]
fn test_is_any_anchored_end_repetition_with_anchor() {
    let literal = Literal::from_char('$');
    let rep = Repetition::new(Hir::literal(literal), 1..5);
    let hir = Hir::repetition(rep);
    hir.is_any_anchored_end();
}

