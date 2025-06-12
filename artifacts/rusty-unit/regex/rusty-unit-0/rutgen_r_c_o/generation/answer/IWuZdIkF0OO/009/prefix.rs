// Answer 0

#[test]
fn test_suffixes_concat_single_anchor_end_text() {
    let mut lits = Literals::empty();
    
    let unicode_char = Literal::Unicode('a');
    let anchor_end_text = Hir::anchor(hir::Anchor::EndText);
    let concat_expr = Hir::concat(vec![anchor_end_text]);
    
    suffixes(&concat_expr, &mut lits);
}

#[test]
fn test_suffixes_concat_single_anchor_end_text_non_complete() {
    let mut lits = Literals::empty();
    
    let unicode_char = Literal::Unicode('a');
    let anchor_end_text = Hir::anchor(hir::Anchor::EndText);
    let literal_expr = Hir::literal(unicode_char);
    let concat_expr = Hir::concat(vec![literal_expr, anchor_end_text]);
    
    suffixes(&concat_expr, &mut lits);
}

#[test]
fn test_suffixes_concat_double_anchor_end_text() {
    let mut lits = Literals::empty();
    
    let unicode_char = Literal::Unicode('b');
    let anchor_end_text = Hir::anchor(hir::Anchor::EndText);
    let literal_expr = Hir::literal(unicode_char);
    let concat_expr = Hir::concat(vec![literal_expr, anchor_end_text]);
    
    suffixes(&concat_expr, &mut lits);
}

#[test]
fn test_suffixes_concat_multiple_elements_non_empty() {
    let mut lits = Literals::empty();
    
    let unicode_char1 = Literal::Unicode('c');
    let unicode_char2 = Literal::Unicode('d');
    let anchor_end_text = Hir::anchor(hir::Anchor::EndText);
    let concat_expr = Hir::concat(vec![Hir::literal(unicode_char1), Hir::literal(unicode_char2), anchor_end_text]);
    
    suffixes(&concat_expr, &mut lits);
}

#[test]
fn test_suffixes_concat_with_empty_literal() {
    let mut lits = Literals::empty();
    
    let empty_literal = Literal::empty();
    let anchor_end_text = Hir::anchor(hir::Anchor::EndText);
    let concat_expr = Hir::concat(vec![Hir::literal(empty_literal), anchor_end_text]);
    
    suffixes(&concat_expr, &mut lits);
}

#[test]
fn test_suffixes_concat_prefixes_non_complete() {
    let mut lits = Literals::empty();
    
    let unicode_char1 = Literal::Unicode('e');
    let anchor_end_text = Hir::anchor(hir::Anchor::EndText);
    let concat_expr = Hir::concat(vec![Hir::literal(unicode_char1), anchor_end_text]);
    
    suffixes(&concat_expr, &mut lits);
}

