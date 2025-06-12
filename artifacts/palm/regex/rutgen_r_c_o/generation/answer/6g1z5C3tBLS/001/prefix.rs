// Answer 0

#[test]
fn test_is_all_assertions_empty() {
    let hir = Hir::empty();
    hir.is_all_assertions();
}

#[test]
fn test_is_all_assertions_single_literal() {
    let literal = Literal::new('a');
    let hir = Hir::literal(literal);
    hir.is_all_assertions();
}

#[test]
fn test_is_all_assertions_anchor() {
    let anchor = Anchor::new();
    let hir = Hir::anchor(anchor);
    hir.is_all_assertions();
}

#[test]
fn test_is_all_assertions_word_boundary() {
    let word_boundary = WordBoundary::new();
    let hir = Hir::word_boundary(word_boundary);
    hir.is_all_assertions();
}

#[test]
fn test_is_all_assertions_repetition() {
    let anchor = Anchor::new();
    let repetition = Repetition::new(Hir::anchor(anchor), 1, None);
    let hir = Hir::repetition(repetition);
    hir.is_all_assertions();
}

#[test]
fn test_is_all_assertions_group() {
    let anchor = Anchor::new();
    let group = Group::new(vec![Hir::anchor(anchor)]);
    let hir = Hir::group(group);
    hir.is_all_assertions();
}

#[test]
fn test_is_all_assertions_concat() {
    let mut exprs = Vec::new();
    for _ in 0..100 {
        let word_boundary = WordBoundary::new();
        exprs.push(Hir::word_boundary(word_boundary));
    }
    let hir = Hir::concat(exprs);
    hir.is_all_assertions();
}

#[test]
fn test_is_all_assertions_alternation() {
    let mut exprs = Vec::new();
    for _ in 0..20 {
        let anchor = Anchor::new();
        exprs.push(Hir::anchor(anchor));
    }
    let hir = Hir::alternation(exprs);
    hir.is_all_assertions();
}

#[test]
fn test_is_all_assertions_combined() {
    let mut exprs = Vec::new();
    let anchor = Anchor::new();
    let group = Group::new(vec![Hir::anchor(anchor)]);
    exprs.push(Hir::group(group));

    let repetition = Repetition::new(Hir::anchor(anchor), 1, None);
    exprs.push(Hir::repetition(repetition));

    let hir = Hir::concat(exprs);
    hir.is_all_assertions();
}

