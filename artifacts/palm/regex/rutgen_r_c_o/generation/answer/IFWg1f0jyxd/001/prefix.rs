// Answer 0

#[test]
fn test_is_match_empty_empty() {
    let hir = Hir::empty();
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_literal() {
    let lit = Literal::from_char('a');
    let hir = Hir::literal(lit);
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_repetition() {
    let lit = Literal::from_char('a');
    let rep = Repetition::zero_or_more(Hir::literal(lit));
    let hir = Hir::repetition(rep);
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_optional() {
    let lit = Literal::from_char('b');
    let opt = Repetition::zero_or_one(Hir::literal(lit));
    let hir = Hir::repetition(opt);
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_group() {
    let group = Group::new(vec![Hir::empty()]);
    let hir = Hir::group(group);
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_concatenation() {
    let lit_a = Literal::from_char('a');
    let lit_b = Literal::from_char('b');
    let hir = Hir::concat(vec![Hir::literal(lit_a), Hir::empty(), Hir::literal(lit_b)]);
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_alternation() {
    let lit_a = Literal::from_char('a');
    let lit_b = Literal::from_char('b');
    let hir = Hir::alternation(vec![Hir::literal(lit_a), Hir::empty(), Hir::literal(lit_b)]);
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_word_boundary() {
    let wb = WordBoundary::new();
    let hir = Hir::word_boundary(wb);
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_anchored() {
    let anchor = Anchor::new_start();
    let hir = Hir::anchor(anchor);
    hir.is_match_empty();
}

#[test]
fn test_is_match_empty_contradictory() {
    let lit = Literal::from_char('c');
    let rep = Repetition::one_or_more(Hir::literal(lit));
    let hir = Hir::repetition(rep);
    hir.is_match_empty();
}

