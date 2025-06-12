// Answer 0

fn test_suffixes_literal_unicode() {
    let unicode_char = 'a';
    let literal = hir::Literal::Unicode(unicode_char);
    let hir = Hir::literal(literal);
    let mut literals = Literals::empty();
    let result = suffixes(&hir, &mut literals);
    assert!(result); // Expect add_char_class_reverse to be successful
}

fn test_suffixes_literal_byte() {
    let byte_value = 0x61; // ASCII for 'a'
    let literal = hir::Literal::Byte(byte_value);
    let hir = Hir::literal(literal);
    let mut literals = Literals::empty();
    let result = suffixes(&hir, &mut literals);
    assert!(result); // Expect cross_add to be successful
}

fn test_suffixes_class_unicode() {
    let cls_unicode = ClassUnicode {
        set: IntervalSet::new(vec![ClassUnicodeRange { start: 'a' as u32, end: 'z' as u32 }]),
    };
    let class = hir::Class::Unicode(cls_unicode);
    let hir = Hir::class(class);
    let mut literals = Literals::empty();
    let result = suffixes(&hir, &mut literals);
    assert!(result); // Expect add_char_class_reverse to be successful
}

fn test_suffixes_class_bytes() {
    let cls_bytes = ClassBytes {
        set: IntervalSet::new(vec![ClassBytesRange { start: 97, end: 122 }]), // 'a' to 'z'
    };
    let class = hir::Class::Bytes(cls_bytes);
    let hir = Hir::class(class);
    let mut literals = Literals::empty();
    let result = suffixes(&hir, &mut literals);
    assert!(result); // Expect add_byte_class to be successful
}

fn test_suffixes_empty_concats() {
    let concat_expr = Hir::concat(vec![]);
    let mut literals = Literals::empty();
    let result = suffixes(&concat_expr, &mut literals);
    assert_eq!(literals.is_empty(), true); // Expect no literals added for empty concat
}

fn test_suffixes_single_concat() {
    let unicode_char = 'b';
    let literal = hir::Literal::Unicode(unicode_char);
    let literal_hir = Hir::literal(literal);
    let concat_expr = Hir::concat(vec![literal_hir]);
    let mut literals = Literals::empty();
    let result = suffixes(&concat_expr, &mut literals);
    assert!(result); // Expect success when having single literal in concat
}

fn test_suffixes_group() {
    let unicode_char = 'c';
    let literal = hir::Literal::Unicode(unicode_char);
    let group_hir = Hir::group(Hir::literal(literal));
    let mut literals = Literals::empty();
    let result = suffixes(&group_hir, &mut literals);
    assert!(result); // Check if suffixes handle groups correctly
}

