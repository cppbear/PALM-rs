// Answer 0

#[test]
fn test_empty_concat_start_text() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![Hir::anchor(hir::Anchor::StartText)]);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_single_anchor_start_text() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![Hir::anchor(hir::Anchor::StartText)]);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_concat_with_start_text() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![
        Hir::anchor(hir::Anchor::StartText),
        Hir::literal(Literal::Unicode('a')),
    ]);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_concat_with_anchor_start_text_and_empty_literal() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![
        Hir::anchor(hir::Anchor::StartText),
        Hir::literal(Literal::empty()),
    ]);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_concat_with_anchor_start_text_and_non_empty_literal() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![
        Hir::anchor(hir::Anchor::StartText),
        Hir::literal(Literal::Unicode('b')),
    ]);
    prefixes(&expr, &mut lits);
}

