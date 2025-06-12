// Answer 0

#[test]
fn test_add_char_class_reverse_normal_case() {
    let mut literals = Literals::empty();
    literals.set_limit_size(500);
    literals.set_limit_class(100);
    
    let cls = ClassUnicode {
        set: IntervalSet::from(vec![(0..10)]), // a valid char range
    };
    
    literals.add(Literal::Unicode('a'));
    literals.add(Literal::Unicode('b'));
    literals.add_char_class_reverse(&cls);
}

#[test]
fn test_add_char_class_reverse_edge_case_limit_exceeded() {
    let mut literals = Literals::empty();
    literals.set_limit_size(1);
    literals.set_limit_class(1);
    
    let cls = ClassUnicode {
        set: IntervalSet::from(vec![(0..300)]), // exceeds character class limit
    };
    
    literals.add(Literal::Unicode('x'));
    let result = literals.add_char_class_reverse(&cls);
}

#[test]
fn test_add_char_class_reverse_empty_lits() {
    let mut literals = Literals::empty();
    literals.set_limit_size(100);
    literals.set_limit_class(50);
    
    let cls = ClassUnicode {
        set: IntervalSet::from(vec![(0..5)]), // small valid range
    };
    
    literals.add_char_class_reverse(&cls);
}

#[test]
fn test_add_char_class_reverse_single_character_class() {
    let mut literals = Literals::empty();
    literals.set_limit_size(100);
    literals.set_limit_class(10);
    
    let cls = ClassUnicode {
        set: IntervalSet::from(vec![(65..66)]), // single valid character: 'A'
    };
    
    literals.add(Literal::Unicode('c'));
    literals.add_char_class_reverse(&cls);
}

#[test]
fn test_add_char_class_reverse_many_lits() {
    let mut literals = Literals::empty();
    literals.set_limit_size(1000);
    literals.set_limit_class(256);
    
    let cls = ClassUnicode {
        set: IntervalSet::from(vec![(0..50)]), // multiple valid characters
    };
    
    for i in 0..1000 {
        literals.add(Literal::Byte(i as u8));
    }
    
    literals.add_char_class_reverse(&cls);
}

