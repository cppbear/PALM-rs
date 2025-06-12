// Answer 0

#[test]
fn test_suffixes_concat_non_empty() {
    let mut lits = Literals::empty();
    lits.set_limit_size(100).set_limit_class(10);
    
    let lit = Literal::Unicode('a');
    lits.add(lit.clone());
    
    let hir = Hir::concat(vec![
        Hir::anchor(hir::Anchor::EndText),
        Hir::literal(lit)
    ]);
    
    suffixes(&hir, &mut lits);
}

#[test]
fn test_suffixes_concat_single_element() {
    let mut lits = Literals::empty();
    lits.set_limit_size(100).set_limit_class(10);
    
    let lit = Literal::Unicode('b');
    lits.add(lit.clone());
    
    let hir = Hir::concat(vec![Hir::literal(lit)]);
    
    suffixes(&hir, &mut lits);
}

#[test]
fn test_suffixes_concat_multiple_elements() {
    let mut lits = Literals::empty();
    lits.set_limit_size(100).set_limit_class(10);
    
    let lit1 = Literal::Unicode('c');
    let lit2 = Literal::Unicode('d');
    lits.add(lit1.clone());
    
    let hir = Hir::concat(vec![Hir::literal(lit1), Hir::literal(lit2)]);
    
    suffixes(&hir, &mut lits);
}

#[test]
fn test_suffixes_concat_with_end_text_anchor() {
    let mut lits = Literals::empty();
    lits.set_limit_size(100).set_limit_class(10);
    
    let lit = Literal::Unicode('e');
    lits.add(lit.clone());
    
    let hir = Hir::concat(vec![
        Hir::anchor(hir::Anchor::EndText),
        Hir::literal(lit)
    ]);
    
    suffixes(&hir, &mut lits);
}

#[test]
fn test_suffixes_concat_empty() {
    let mut lits = Literals::empty();
    lits.set_limit_size(100).set_limit_class(10);
    
    let hir = Hir::concat(vec![]);
    
    suffixes(&hir, &mut lits);
}

