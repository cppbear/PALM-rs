// Answer 0

#[test]
fn test_add_literal_at_limit_size() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 5,
        limit_class: 0,
    };
    
    literals.lits.push(Literal::Unicode('a')); // 1 byte
    literals.lits.push(Literal::Unicode('b')); // 1 byte
    literals.lits.push(Literal::Unicode('c')); // 1 byte
    literals.lits.push(Literal::Unicode('d')); // 1 byte
    literals.lits.push(Literal::Unicode('e')); // 1 byte

    let new_literal = Literal::Unicode('f'); // 1 byte
    let result = literals.add(new_literal);

    // result should be true as num_bytes is exactly equal to limit_size
}

#[test]
fn test_add_literal_exact_limit_size() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 3,
        limit_class: 0,
    };
    
    literals.lits.push(Literal::Unicode('x')); // 1 byte
    literals.lits.push(Literal::Unicode('y')); // 1 byte

    let new_literal = Literal::Unicode('z'); // 1 byte
    let result = literals.add(new_literal);

    // result should be true as num_bytes is 2 and new literal makes it 3
} 

#[test]
fn test_add_large_literal_within_limit_size() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 0,
    };

    literals.lits.push(Literal::Unicode('1')); // 1 byte
    literals.lits.push(Literal::Unicode('2')); // 1 byte
    literals.lits.push(Literal::Unicode('3')); // 1 byte
    literals.lits.push(Literal::Unicode('4')); // 1 byte

    let new_literal = Literal::Unicode('5'); // 1 byte
    let result = literals.add(new_literal);

    // result should be true as num_bytes is 4 and new literal makes it 5
}

#[test]
fn test_add_empty_literal_to_non_empty() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 4,
        limit_class: 0,
    };

    literals.lits.push(Literal::Unicode('a')); // 1 byte
    let new_literal = Literal::Unicode('b'); // 1 byte
    let result = literals.add(new_literal);

    // result should be true as it is within the limit size 
}

#[test]
fn test_add_small_literal_to_full() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 5,
        limit_class: 0,
    };
    
    literals.lits.push(Literal::Unicode('a')); // 1 byte
    literals.lits.push(Literal::Unicode('b')); // 1 byte
    literals.lits.push(Literal::Unicode('c')); // 1 byte
    literals.lits.push(Literal::Unicode('d')); // 1 byte
    literals.lits.push(Literal::Unicode('e')); // 1 byte

    let new_literal = Literal::Unicode('f'); // 1 byte
    let result = literals.add(new_literal);

    // result should be false as adding this literal exceeds the limit size
}

