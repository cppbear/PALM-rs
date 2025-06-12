// Answer 0

#[test]
fn test_prefixes_empty_literal() {
    use hir::{Hir, HirKind};
    let mut lits = Literals::empty();
    let expr = Hir::literal(Literal::empty());
    prefixes(&expr, &mut lits);
    assert!(lits.is_empty());
}

#[test]
fn test_prefixes_single_unicode_literal() {
    use hir::{Hir, HirKind, Literal};
    let mut lits = Literals::empty();
    let expr = Hir::literal(Literal::Unicode('a'));
    prefixes(&expr, &mut lits);
    assert_eq!(lits.literals().len(), 1);
    assert_eq!(lits.literals()[0], Literal::new(vec![b'a']));
}

#[test]
fn test_prefixes_single_byte_literal() {
    use hir::{Hir, HirKind, Literal};
    let mut lits = Literals::empty();
    let expr = Hir::literal(Literal::Byte(b'a'));
    prefixes(&expr, &mut lits);
    assert_eq!(lits.literals().len(), 1);
    assert_eq!(lits.literals()[0], Literal::new(vec![b'a']));
}

#[test]
fn test_prefixes_unicode_class() {
    use hir::{Hir, HirKind, Class, ClassUnicode, Literal};
    let mut lits = Literals::empty();
    let cls = ClassUnicode { set: IntervalSet::new(vec![0..128]) }; // ASCII range
    let expr = Hir::class(Class::Unicode(cls));
    prefixes(&expr, &mut lits);
    assert!(lits.literals().len() > 0);
}

#[test]
fn test_prefixes_bytes_class() {
    use hir::{Hir, HirKind, Class, ClassBytes, Literal};
    let mut lits = Literals::empty();
    let cls = ClassBytes { set: IntervalSet::new(vec![b'a'..=b'z']) }; // ASCII lowercase range
    let expr = Hir::class(Class::Bytes(cls));
    prefixes(&expr, &mut lits);
    assert!(lits.literals().len() > 0);
}

#[test]
fn test_prefixes_group() {
    use hir::{Hir, HirKind, Group, Literal};
    let mut lits = Literals::empty();
    let inner_expr = Hir::literal(Literal::Unicode('b'));
    let expr = Hir::group(Group { hir: Box::new(inner_expr), capturing: false });
    prefixes(&expr, &mut lits);
    assert_eq!(lits.literals().len(), 1);
    assert_eq!(lits.literals()[0], Literal::new(vec![b'b']));
}

#[test]
fn test_prefixes_concat_non_empty() {
    use hir::{Hir, HirKind, Group, Literal};
    let mut lits = Literals::empty();
    let expr1 = Hir::literal(Literal::Unicode('c'));
    let expr2 = Hir::literal(Literal::Unicode('d'));
    let expr = Hir::concat(vec![expr1, expr2]);
    prefixes(&expr, &mut lits);
    assert_eq!(lits.literals().len(), 1);
    assert_eq!(lits.literals()[0], Literal::new(vec![b'c', b'd']));
}

#[test]
fn test_prefixes_concat_with_empty() {
    use hir::{Hir, HirKind, Group, Literal};
    let mut lits = Literals::empty();
    let expr1 = Hir::literal(Literal::Unicode('e'));
    let expr2 = Hir::literal(Literal::empty());
    let expr = Hir::concat(vec![expr1, expr2]);
    prefixes(&expr, &mut lits);
    assert_eq!(lits.literals().len(), 1);
    assert_eq!(lits.literals()[0], Literal::new(vec![b'e']));
}

