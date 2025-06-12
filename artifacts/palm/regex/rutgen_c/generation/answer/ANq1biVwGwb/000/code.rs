// Answer 0

#[test]
fn test_deref() {
    let literal = Literal {
        v: vec![1, 2, 3, 4, 5],
        cut: false,
    };
    let deref_result: &Vec<u8> = &literal;
    assert_eq!(deref_result, &vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_deref_empty() {
    let literal = Literal {
        v: vec![],
        cut: true,
    };
    let deref_result: &Vec<u8> = &literal;
    assert_eq!(deref_result, &vec![]);
}

#[test]
fn test_deref_multiple() {
    let literal = Literal {
        v: vec![10, 20, 30],
        cut: true,
    };
    let deref_result: &Vec<u8> = &literal;
    assert_eq!(deref_result, &vec![10, 20, 30]);
}

