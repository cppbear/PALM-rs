// Answer 0

#[test]
fn test_add_char_class_exceeds_limits() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 1,
        limit_class: 1,
    };

    let class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'b' }
    ]);

    let result = literals._add_char_class(&class_unicode, false);
}

#[test]
fn test_add_char_class_exceeds_limits_reverse() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 1,
        limit_class: 1,
    };

    let class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'c', end: 'd' }
    ]);

    let result = literals._add_char_class(&class_unicode, true);
}

