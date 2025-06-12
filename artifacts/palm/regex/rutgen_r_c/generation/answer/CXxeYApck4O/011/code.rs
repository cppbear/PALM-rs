// Answer 0

#[test]
fn test_prefixes_with_empty_literals_and_anchor_start_text() {
    let anchor = Hir::anchor(hir::Anchor::StartText);
    let concat_hir = Hir::concat(vec![anchor]);
    let mut literals = Literals::empty();

    prefixes(&concat_hir, &mut literals);

    assert!(literals.is_empty());
}

#[test]
fn test_prefixes_with_single_literal_and_empty_lits() {
    let literal = Hir::literal(Literal::Unicode('a'));
    let concat_hir = Hir::concat(vec![literal]);
    let mut literals = Literals::empty();

    prefixes(&concat_hir, &mut literals);

    assert!(literals.is_empty());
}

#[test]
fn test_prefixes_with_literal_byte_and_empty_lits() {
    let byte_literal = Hir::literal(Literal::Byte(b'a'));
    let concat_hir = Hir::concat(vec![byte_literal]);
    let mut literals = Literals::empty();

    prefixes(&concat_hir, &mut literals);

    assert!(literals.is_empty());
}

#[test]
fn test_prefixes_with_unicode_class_and_empty_lits() {
    let unicode_class = Class::Unicode(ClassUnicode { set: IntervalSet::default() });
    let class_hir = Hir::class(unicode_class);
    let concat_hir = Hir::concat(vec![class_hir]);
    let mut literals = Literals::empty();

    prefixes(&concat_hir, &mut literals);

    assert!(literals.is_empty());
}

#[test]
fn test_prefixes_with_byte_class_and_empty_lits() {
    let byte_class = Class::Bytes(ClassBytes { set: IntervalSet::default() });
    let class_hir = Hir::class(byte_class);
    let concat_hir = Hir::concat(vec![class_hir]);
    let mut literals = Literals::empty();

    prefixes(&concat_hir, &mut literals);

    assert!(literals.is_empty());
}

#[test]
fn test_prefixes_with_empty_concat() {
    let concat_hir = Hir::concat(vec![]);
    let mut literals = Literals::empty();

    prefixes(&concat_hir, &mut literals);

    assert!(literals.is_empty());
}

