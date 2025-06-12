// Answer 0

#[test]
fn test_suffixes_literal_unicode() {
    let unicode_char = 'a';
    let literal = Literal::Unicode(unicode_char);
    let hir = Hir::literal(literal);
    let mut lits = Literals::empty();
    lits.set_limit_size(1024);
    lits.set_limit_class(256);
    suffixes(&hir, &mut lits);
}

#[test]
fn test_suffixes_literal_byte() {
    let byte_value: u8 = 65; // ASCII 'A'
    let literal = Literal::Byte(byte_value);
    let hir = Hir::literal(literal);
    let mut lits = Literals::empty();
    lits.set_limit_size(1024);
    lits.set_limit_class(256);
    suffixes(&hir, &mut lits);
}

#[test]
fn test_suffixes_class_unicode() {
    let class_unicode = ClassUnicode {
        set: IntervalSet::new(vec![ClassUnicodeRange::new(0x61, 0x7A)]), // 'a' to 'z'
    };
    let hir = Hir::class(Class::Unicode(class_unicode));
    let mut lits = Literals::empty();
    lits.set_limit_size(1024);
    lits.set_limit_class(256);
    suffixes(&hir, &mut lits);
}

#[test]
fn test_suffixes_class_bytes() {
    let class_bytes = ClassBytes {
        set: IntervalSet::new(vec![ClassBytesRange::new(0x41, 0x5A)]), // 'A' to 'Z'
    };
    let hir = Hir::class(Class::Bytes(class_bytes));
    let mut lits = Literals::empty();
    lits.set_limit_size(1024);
    lits.set_limit_class(256);
    suffixes(&hir, &mut lits);
}

#[test]
fn test_suffixes_concat_with_literal() {
    let literal = Literal::Unicode('b');
    let hir = Hir::concat(vec![Hir::literal(literal)]);
    let mut lits = Literals::empty();
    lits.set_limit_size(1024);
    lits.set_limit_class(256);
    suffixes(&hir, &mut lits);
}

#[test]
fn test_suffixes_concat_with_class_unicode() {
    let class_unicode = ClassUnicode {
        set: IntervalSet::new(vec![ClassUnicodeRange::new(0x61, 0x7A)]), // 'a' to 'z'
    };
    let hir = Hir::concat(vec![Hir::class(Class::Unicode(class_unicode))]);
    let mut lits = Literals::empty();
    lits.set_limit_size(1024);
    lits.set_limit_class(256);
    suffixes(&hir, &mut lits);
}

