// Answer 0

#[test]
fn test_choose_multiple_with_positive_amount() {
    let source = vec![1, 2, 3, 4, 5];
    let result = choose_multiple(source, 3);
    assert_eq!(result.len(), 3);
}

#[test]
fn test_choose_multiple_with_zero_amount() {
    let source = vec![1, 2, 3, 4, 5];
    let result = choose_multiple(source, 0);
    assert_eq!(result.len(), 0);
}

#[should_panic]
fn test_choose_multiple_with_amount_greater_than_source() {
    let source = vec![1, 2, 3];
    let _result = choose_multiple(source, 5);
}

#[test]
fn test_choose_multiple_with_empty_source() {
    let source: Vec<i32> = vec![];
    let result = choose_multiple(source, 0);
    assert_eq!(result.len(), 0);
}

