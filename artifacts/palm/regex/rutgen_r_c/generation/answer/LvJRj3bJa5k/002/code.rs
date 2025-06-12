// Answer 0

#[test]
fn test_trim_suffix_some() {
    let mut literals = {
        let mut lits = Literals::empty();
        lits.set_limit_size(10);
        lits.set_limit_class(5);
        lits.lits.push(Literal::Byte(1));
        lits.lits.push(Literal::Byte(2));
        lits.lits.push(Literal::Byte(3));
        lits
    };

    let result = literals.trim_suffix(1);
    assert!(result.is_some());
    
    let trimmed_literals = result.unwrap();
    assert_eq!(trimmed_literals.lits.len(), 3);
    assert_eq!(trimmed_literals.lits[0], Literal::Byte(1));
    assert_eq!(trimmed_literals.lits[1], Literal::Byte(2));
    assert_eq!(trimmed_literals.lits[2], Literal::Byte(3));
}

#[test]
fn test_trim_suffix_none() {
    let literals = {
        let mut lits = Literals::empty();
        lits.set_limit_size(5);
        lits.set_limit_class(3);
        lits.lits.push(Literal::Byte(1));
        lits.lits.push(Literal::Byte(2));
        lits.lits.push(Literal::Byte(3));
        lits
    };

    let result = literals.trim_suffix(5);
    assert!(result.is_none());
}

#[test]
fn test_trim_suffix_empty_case() {
    let literals = {
        let mut lits = Literals::empty();
        lits.set_limit_size(5);
        lits.set_limit_class(3);
        lits
    };

    let result = literals.trim_suffix(1);
    assert!(result.is_none());
}

#[test]
fn test_trim_suffix_duplicate_removal() {
    let mut literals = {
        let mut lits = Literals::empty();
        lits.set_limit_size(10);
        lits.set_limit_class(5);
        lits.lits.push(Literal::Byte(1));
        lits.lits.push(Literal::Byte(2));
        lits.lits.push(Literal::Byte(2));
        lits.lits.push(Literal::Byte(3));
        lits.lits.push(Literal::Byte(3));
        lits
    };

    let result = literals.trim_suffix(1);
    assert!(result.is_some());
    
    let trimmed_literals = result.unwrap();
    assert_eq!(trimmed_literals.lits.len(), 3);
    assert_eq!(trimmed_literals.lits[0], Literal::Byte(1));
    assert_eq!(trimmed_literals.lits[1], Literal::Byte(2));
    assert_eq!(trimmed_literals.lits[2], Literal::Byte(3));
}

#[test]
fn test_trim_suffix_almost_empty() {
    let literals = {
        let mut lits = Literals::empty();
        lits.set_limit_size(5);
        lits.set_limit_class(3);
        lits.lits.push(Literal::Byte(1));
        lits.lits.push(Literal::Byte(2));
        lits
    };

    let result = literals.trim_suffix(2);
    assert!(result.is_none());
}

