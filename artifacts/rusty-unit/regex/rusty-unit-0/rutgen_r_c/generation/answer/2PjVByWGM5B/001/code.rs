// Answer 0

#[test]
fn test_add_char_class_exceeds_limits() {
    use std::char;
    use hir::{ClassUnicode, ClassUnicodeRange, Literal};

    // Initializing a Literals struct with limit_size and limit_class that are low.
    let mut literals = Literals {
        lits: vec![Literal::Byte(0)], // starting with a single byte literal
        limit_size: 1, // set to a low limit
        limit_class: 2, // also a low limit
    };
    
    let range_a = ClassUnicodeRange { start: 'a', end: 'b' }; // corresponds to 2 characters 'a' and 'b'
    let class_unicode = ClassUnicode::new(vec![range_a]);

    // The class_unicode will have a character count that exceeds limits when added
    let result = literals._add_char_class(&class_unicode, false);
    
    assert_eq!(result, false);
}

#[test]
fn test_add_char_class_empty_base() {
    use hir::{ClassUnicode, ClassUnicodeRange, Literal};

    // Initializing a Literals struct with an empty set of literals
    let mut literals = Literals {
        lits: vec![],
        limit_size: 5, // arbitrary limit
        limit_class: 5, // arbitrary limit
    };
    
    let range_a = ClassUnicodeRange { start: 'a', end: 'b' }; // corresponds to 2 characters 'a' and 'b'
    let class_unicode = ClassUnicode::new(vec![range_a]);

    // This should succeed since class does not exceed limits with an empty base
    let result = literals._add_char_class(&class_unicode, false);
    
    assert_eq!(result, true);
    assert_eq!(literals.lits.len(), 2); // Expect two literals for 'a' and 'b'
}

#[test]
fn test_add_char_class_reverse() {
    use hir::{ClassUnicode, ClassUnicodeRange, Literal};

    // Initializing a Literals struct with enough space
    let mut literals = Literals {
        lits: vec![Literal::Byte(0)],
        limit_size: 10,
        limit_class: 10,
    };
    
    let range_b = ClassUnicodeRange { start: 'b', end: 'c' }; // corresponds to 'b' and 'c'
    let class_unicode = ClassUnicode::new(vec![range_b]);

    // Test the reverse flag to ensure characters are added in reverse order
    let result = literals._add_char_class(&class_unicode, true);

    assert_eq!(result, true);
    assert_eq!(literals.lits.len(), 2); // Expect two literals for 'b' and 'c'
    assert_eq!(literals.lits[0], Literal::Byte(b'b')); // Check if byte is correct
    assert_eq!(literals.lits[1], Literal::Byte(b'c'));
}

