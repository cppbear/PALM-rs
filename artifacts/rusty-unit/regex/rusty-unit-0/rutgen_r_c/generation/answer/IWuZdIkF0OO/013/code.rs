// Answer 0

fn test_suffixes_empty_concat() {
    let expr = Hir::concat(vec![]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
    assert!(lits.is_empty());
}

fn test_suffixes_single_literal() {
    let expr = Hir::literal(Literal::Unicode('a'));
    let mut lits = Literals::empty();
    assert!(lits.cross_add(b"a"));
    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

fn test_suffixes_concat_one() {
    let lit = Hir::literal(Literal::Unicode('b'));
    let expr = Hir::concat(vec![lit]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty()); 
}

fn test_suffixes_concat_multiple() {
    let lit_a = Hir::literal(Literal::Unicode('a'));
    let lit_b = Hir::literal(Literal::Byte(b'b'));
    let expr = Hir::concat(vec![lit_a, lit_b]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

fn test_suffixes_concat_cut_case() {
    let lit = Hir::literal(Literal::Unicode('c'));
    let expr = Hir::concat(vec![lit]);
    let mut lits = Literals::empty();
    lits.cut(); 
    suffixes(&expr, &mut lits);
    assert!(lits.is_empty());
}

fn test_suffixes_concat_anchor_end() {
    let lit = Hir::literal(Literal::Unicode('d'));
    let end_anchor = Hir::anchor(hir::Anchor::EndText);
    let expr = Hir::concat(vec![lit, end_anchor]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
    assert!(lits.is_empty());
}

fn test_suffixes_concat_complete() {
    let lit = Hir::literal(Literal::Unicode('e'));
    let expr = Hir::concat(vec![lit]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

fn test_suffixes_concat_alternation() {
    let lit = Hir::literal(Literal::Unicode('f'));
    let alt = Hir::alternation(vec![lit]);
    let expr = Hir::concat(vec![alt]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

fn test_suffixes_concat_with_cut() {
    let lit = Hir::literal(Literal::Byte(b'g'));
    let expr = Hir::concat(vec![lit]);
    let mut lits = Literals::empty();
    lits.cut(); 
    suffixes(&expr, &mut lits);
    assert!(lits.is_empty());
}

