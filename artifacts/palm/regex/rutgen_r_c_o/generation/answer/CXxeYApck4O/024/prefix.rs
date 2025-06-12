// Answer 0

#[test]
fn test_prefixes_unicode_class() {
    let mut lits = Literals::empty();
    lits.set_limit_size(1000);
    lits.set_limit_class(10);
    
    let unicode_class = ClassUnicode {
        set: IntervalSet::new(vec![ClassUnicodeRange::new(65, 90)], false),
    };
    let expr = Hir::class(Class::Unicode(unicode_class));
    
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_bytes_class() {
    let mut lits = Literals::empty();
    lits.set_limit_size(1000);
    lits.set_limit_class(20);
    
    let bytes_class = ClassBytes {
        set: IntervalSet::new(vec![ClassBytesRange::new(0, 255)], false),
    };
    let expr = Hir::class(Class::Bytes(bytes_class));
    
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_single_unicode_literal() {
    let mut lits = Literals::empty();
    lits.set_limit_size(1000);
    lits.set_limit_class(5);
    
    let lit = Literal::Unicode('A');
    let expr = Hir::literal(lit);
    
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_single_byte_literal() {
    let mut lits = Literals::empty();
    lits.set_limit_size(1000);
    lits.set_limit_class(3);
    
    let lit = Literal::Byte(65);
    let expr = Hir::literal(lit);
    
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_empty_concat() {
    let mut lits = Literals::empty();
    lits.set_limit_size(1000);
    lits.set_limit_class(10);
    
    let expr = Hir::concat(vec![]);
    
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_single_group() {
    let mut lits = Literals::empty();
    lits.set_limit_size(1000);
    lits.set_limit_class(8);
    
    let lit = Literal::Unicode('B');
    let expr = Hir::group(Hir::literal(lit));
    
    prefixes(&expr, &mut lits);
}

