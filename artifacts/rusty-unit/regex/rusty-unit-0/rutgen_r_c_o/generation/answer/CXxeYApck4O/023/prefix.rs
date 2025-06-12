// Answer 0

#[test]
fn test_prefixes_with_unicode_literal() {
    let unicode_char = 'a';
    let mut lits = Literals::empty();
    lits.set_limit_size(10);
    lits.set_limit_class(5);
    let expr = Hir::literal(Literal::Unicode(unicode_char));
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_with_byte_literal() {
    let byte_value = 5u8;
    let mut lits = Literals::empty();
    lits.set_limit_size(10);
    lits.set_limit_class(5);
    let expr = Hir::literal(Literal::Byte(byte_value));
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_with_unicode_class() {
    let cls = Class::Unicode(ClassUnicode {
        set: IntervalSet::empty(),
    });
    let mut lits = Literals::empty();
    lits.set_limit_size(100);
    lits.set_limit_class(10);
    let expr = Hir::class(cls);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_with_byte_class_exceeding_limit() {
    let byte_range = ClassBytes {
        set: IntervalSet::of(vec![ClassBytesRange {
            start: 0,
            end: 255,
        }]),
    };
    let mut lits = Literals::empty();
    lits.set_limit_size(5);
    lits.set_limit_class(3);
    let expr = Hir::class(Class::Bytes(byte_range));
    prefixes(&expr, &mut lits);
}

