// Answer 0

#[test]
fn test_suffixes_unicode_literal() {
    let unicode_char = 'a';
    let literal = Literal::Unicode(unicode_char);
    let hir = Hir::literal(literal);
    
    let mut lits = Literals::empty();
    suffixes(&hir, &mut lits);
    
    assert!(lits.is_empty());
}

#[test]
fn test_suffixes_byte_literal() {
    let byte = 97; // ASCII for 'a'
    let literal = Literal::Byte(byte);
    let hir = Hir::literal(literal);
    
    let mut lits = Literals::empty();
    suffixes(&hir, &mut lits);
    
    assert!(lits.is_empty());
}

#[test]
fn test_suffixes_unicode_class_failure() {
    let cls = Class::Unicode(ClassUnicode { set: IntervalSet::new() }); // empty class
    let hir = Hir::class(cls);
    
    let mut lits = Literals::empty();
    suffixes(&hir, &mut lits);
    
    assert!(lits.is_empty());
}

#[test]
fn test_suffixes_byte_class_failure() {
    let cls = Class::Bytes(ClassBytes { set: IntervalSet::new() }); // empty class
    let hir = Hir::class(cls);
    
    let mut lits = Literals::empty();
    suffixes(&hir, &mut lits);
    
    assert!(lits.is_empty());
}

#[test]
fn test_suffixes_concatenation_empty() {
    let hir = Hir::concat(vec![]); // empty concat
    
    let mut lits = Literals::empty();
    suffixes(&hir, &mut lits);
    
    assert!(lits.is_empty());
}

#[test]
fn test_suffixes_concatenation_with_lit() {
    let byte_lit = Literal::Byte(98); // ASCII for 'b'
    let byte_hir = Hir::literal(byte_lit);
    
    let unicode_lit = Literal::Unicode('c');
    let unicode_hir = Hir::literal(unicode_lit);
    
    let hir = Hir::concat(vec![byte_hir, unicode_hir]);
    
    let mut lits = Literals::empty();
    suffixes(&hir, &mut lits);
    
    assert!(lits.is_empty());
}

