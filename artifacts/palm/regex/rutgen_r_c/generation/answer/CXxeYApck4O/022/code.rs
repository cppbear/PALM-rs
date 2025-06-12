// Answer 0

#[test]
fn test_prefixes_literal_unicode() {
    let unicode_literal = hir::Literal::Unicode('a');
    let literal = hir::Literal::new(vec![b'a']);
    let expr = hir::Hir::literal(literal);
    let mut lits = Literals::empty();
    lits.set_limit_size(10);
    
    prefixes(&expr, &mut lits);
    
    assert_eq!(lits.literals().len(), 1);
    assert_eq!(lits.literals()[0], Literal::new(vec![b'a']));
}

#[test]
fn test_prefixes_literal_byte() {
    let byte_literal = hir::Literal::Byte(b'b');
    let literal = hir::Literal::new(vec![b'b']);
    let expr = hir::Hir::literal(literal);
    let mut lits = Literals::empty();
    lits.set_limit_size(10);
    
    prefixes(&expr, &mut lits);
    
    assert_eq!(lits.literals().len(), 1);
    assert_eq!(lits.literals()[0], Literal::new(vec![b'b']));
}

#[test]
fn test_prefixes_class_unicode() {
    let unicode_class = hir::Class::Unicode(ClassUnicode {
        set: IntervalSet::new(),
    });
    let expr = hir::Hir::class(unicode_class);
    let mut lits = Literals::empty();
    lits.set_limit_size(10);
    
    let result = lits.add_char_class(&unicode_class);
    prefixes(&expr, &mut lits);
    
    assert!(result);
    assert!(lits.any_complete());
}

#[test]
fn test_prefixes_class_bytes() {
    let byte_class = hir::Class::Bytes(ClassBytes {
        set: IntervalSet::new(),
    });
    let expr = hir::Hir::class(byte_class);
    let mut lits = Literals::empty();
    lits.set_limit_size(10);
    
    let result = lits.add_byte_class(&byte_class);
    prefixes(&expr, &mut lits);
    
    assert!(result);
    assert!(lits.any_complete());
}

#[test]
fn test_prefixes_group() {
    let unicode_literal = hir::Literal::Unicode('c');
    let literal = hir::Literal::new(vec![b'c']);
    let inner_expr = hir::Hir::literal(literal);
    let group_expr = hir::Hir::group(hir::Group { hir: Box::new(inner_expr) });
    let mut lits = Literals::empty();
    lits.set_limit_size(10);
    
    prefixes(&group_expr, &mut lits);
    
    assert_eq!(lits.literals().len(), 1);
    assert_eq!(lits.literals()[0], Literal::new(vec![b'c']));
}

