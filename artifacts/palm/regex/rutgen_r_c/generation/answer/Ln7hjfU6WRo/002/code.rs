// Answer 0

#[test]
fn test_longest_common_prefix_non_empty() {
    let lit1 = Literal::Unicode('a');
    let lit2 = Literal::Unicode('a');
    let lit3 = Literal::Unicode('a');
    
    let literals = Literals {
        lits: vec![
            lit1.clone(),
            lit2.clone(),
            lit3.clone()
        ],
        limit_size: 10,
        limit_class: 5,
    };

    assert_eq!(literals.longest_common_prefix(), b"a");
}

#[test]
fn test_longest_common_prefix_empty_strings() {
    let lit1 = Literal::Unicode('a');
    let lit2 = Literal::Unicode('b');
    
    let literals = Literals {
        lits: vec![
            lit1.clone(),
            lit2.clone(),
        ],
        limit_size: 10,
        limit_class: 5,
    };

    assert_eq!(literals.longest_common_prefix(), b"");
}

#[test]
fn test_longest_common_prefix_mixed() {
    let lit1 = Literal::Byte(b'a');
    let lit2 = Literal::Byte(b'a');
    let lit3 = Literal::Byte(b'a');
    let lit4 = Literal::Byte(b'b');
    
    let literals = Literals {
        lits: vec![
            lit1.clone(),
            lit2.clone(),
            lit3.clone(),
            lit4.clone()
        ],
        limit_size: 10,
        limit_class: 5,
    };

    assert_eq!(literals.longest_common_prefix(), b"a");
}

#[test]
fn test_longest_common_prefix_single_literal() {
    let lit1 = Literal::Unicode('c');

    let literals = Literals {
        lits: vec![
            lit1.clone()
        ],
        limit_size: 10,
        limit_class: 5,
    };

    assert_eq!(literals.longest_common_prefix(), b"c");
}

#[test]
#[should_panic]
fn test_longest_common_prefix_empty_literals() {
    let literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 5,
    };

    // This should panic since we are not allowed to call on empty literals
    literals.longest_common_prefix();
}

