// Answer 0

#[test]
fn test_add_char_class_with_valid_inputs() {
    let mut literals = Literals {
        lits: vec![Literal::Byte(97)], // base is not empty
        limit_size: 1024,
        limit_class: 65535,
    };
    
    let range = ClassUnicodeRange { start: 'a', end: 'c' }; // valid range
    let class_unicode = ClassUnicode::new(vec![range]);

    assert!(literals._add_char_class(&class_unicode, false)); 
}

#[test]
fn test_add_char_class_with_empty_class_unicode() {
    let mut literals = Literals {
        lits: vec![Literal::Byte(97)], // base is not empty
        limit_size: 1024,
        limit_class: 65535,
    };
    
    let class_unicode = ClassUnicode::empty();

    assert!(literals._add_char_class(&class_unicode, false)); 
}

#[test]
fn test_add_char_class_with_upper_character_range() {
    let mut literals = Literals {
        lits: vec![Literal::Byte(98)], // base is not empty
        limit_size: 1024,
        limit_class: 65535,
    };
    
    let range = ClassUnicodeRange { start: 'A', end: 'C' }; // valid range with uppercase letters
    let class_unicode = ClassUnicode::new(vec![range]);

    assert!(literals._add_char_class(&class_unicode, false)); 
}

#[test]
fn test_add_char_class_with_multiple_ranges() {
    let mut literals = Literals {
        lits: vec![Literal::Byte(99)], // base is not empty
        limit_size: 1024,
        limit_class: 65535,
    };
    
    let ranges = vec![
        ClassUnicodeRange { start: 'd', end: 'f' },
        ClassUnicodeRange { start: 'g', end: 'j' },
    ];
    let class_unicode = ClassUnicode::new(ranges);

    assert!(literals._add_char_class(&class_unicode, false)); 
}

#[test]
fn test_add_char_class_with_reverse_false() {
    let mut literals = Literals {
        lits: vec![Literal::Byte(100)], // base is not empty
        limit_size: 1024,
        limit_class: 65535,
    };
    
    let range = ClassUnicodeRange { start: 'x', end: 'y' };
    let class_unicode = ClassUnicode::new(vec![range]);

    assert!(literals._add_char_class(&class_unicode, false)); 
}

#[test]
fn test_add_char_class_with_limit_class_exceeded() {
    let mut literals = Literals {
        lits: vec![], // base is empty
        limit_size: 1024,
        limit_class: 1, // intentionally low to exceed limit
    };
    
    let range = ClassUnicodeRange { start: 'a', end: 'c' }; // valid range
    let class_unicode = ClassUnicode::new(vec![range]);

    assert!(!literals._add_char_class(&class_unicode, false)); 
}

