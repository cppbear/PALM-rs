// Answer 0

#[test]
fn test_reverse_empty_literals() {
    let mut literals = Literals::empty();
    literals.reverse();
    assert!(literals.is_empty());
}

#[test]
fn test_reverse_single_literal_unicode() {
    let mut literals = Literals { 
        lits: vec![Literal::Unicode('a')], 
        limit_size: 0, 
        limit_class: 0 
    };
    literals.reverse();
    assert_eq!(literals.literals(), vec![&Literal::Unicode('a')]);
}

#[test]
fn test_reverse_single_literal_byte() {
    let mut literals = Literals { 
        lits: vec![Literal::Byte(97)], 
        limit_size: 0, 
        limit_class: 0 
    };
    literals.reverse();
    assert_eq!(literals.literals(), vec![&Literal::Byte(97)]);
}

#[test]
fn test_reverse_multiple_literals() {
    let mut literals = Literals { 
        lits: vec![Literal::Unicode('a'), Literal::Unicode('b'), Literal::Unicode('c')], 
        limit_size: 0, 
        limit_class: 0 
    };
    literals.reverse();
    assert_eq!(literals.literals(), vec![&Literal::Unicode('c'), &Literal::Unicode('b'), &Literal::Unicode('a')]);
} 

#[test]
fn test_reverse_liters_with_bytes() {
    let mut literals = Literals { 
        lits: vec![Literal::Byte(97), Literal::Unicode('b'), Literal::Byte(99)], 
        limit_size: 0, 
        limit_class: 0 
    };
    literals.reverse();
    assert_eq!(literals.literals(), vec![&Literal::Byte(99), &Literal::Unicode('b'), &Literal::Byte(97)]);
}

