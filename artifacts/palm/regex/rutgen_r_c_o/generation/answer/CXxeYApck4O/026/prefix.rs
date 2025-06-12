// Answer 0

#[test]
fn test_prefixes_empty() {
    let expr = Hir::empty();
    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_word_boundary() {
    let expr = Hir::word_boundary(WordBoundary {});
    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_anchor() {
    let expr = Hir::anchor(Anchor {});
    let mut lits = Literals::empty();
    prefixes(&expr, &mut lits);
}

