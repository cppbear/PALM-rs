// Answer 0

#[test]
fn test_prefixes_concat_with_anchor_start_text() {
    let mut lits = Literals::empty().set_limit_size(1).set_limit_class(1);
    let anchor_start_text = Hir::anchor(hir::Anchor::StartText);
    let literal = Hir::literal(Literal::Unicode('a'));
    let concat_expr = Hir::concat(vec![anchor_start_text, literal]);

    prefixes(&concat_expr, &mut lits);
}

#[test]
fn test_prefixes_concat_empty() {
    let mut lits = Literals::empty().set_limit_size(1).set_limit_class(1);
    let empty_concat = Hir::concat(vec![]);

    prefixes(&empty_concat, &mut lits);
}

#[test]
fn test_prefixes_concat_one_element() {
    let mut lits = Literals::empty().set_limit_size(1).set_limit_class(1);
    let anchor_start_text = Hir::anchor(hir::Anchor::StartText);
    let concat_one_element = Hir::concat(vec![anchor_start_text]);

    prefixes(&concat_one_element, &mut lits);
}

#[test]
fn test_prefixes_concat_multiple_elements_with_anchor() {
    let mut lits = Literals::empty().set_limit_size(1).set_limit_class(1);
    let anchor_start_text = Hir::anchor(hir::Anchor::StartText);
    let literal_b = Hir::literal(Literal::Byte(98));
    let concat_multiple_elements = Hir::concat(vec![anchor_start_text, literal_b]);

    prefixes(&concat_multiple_elements, &mut lits);
}

#[test]
fn test_prefixes_concat_with_class() {
    let mut lits = Literals::empty().set_limit_size(1).set_limit_class(1);
    let anchor_start_text = Hir::anchor(hir::Anchor::StartText);
    let class_unicode = Hir::class(Class::Unicode(ClassUnicode { set: IntervalSet::new() }));
    let concat_with_class = Hir::concat(vec![anchor_start_text, class_unicode]);

    prefixes(&concat_with_class, &mut lits);
}

