// Answer 0

#[test]
fn test_choose_multiple_empty_source() {
    let source: Vec<i32> = Vec::new();
    let result = choose_multiple(source, 1);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_choose_multiple_amount_zero() {
    let source = vec![1, 2, 3, 4, 5];
    let result = choose_multiple(source, 0);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_choose_multiple_amount_greater_than_length() {
    let source = vec![1, 2, 3];
    let result = choose_multiple(source.clone(), 5);
    assert!(result.len() > 0 && result.len() <= source.len());
}

#[test]
fn test_choose_multiple_exactly_five() {
    let source = vec![1, 2, 3, 4, 5];
    let result = choose_multiple(source.clone(), 5);
    assert_eq!(result, source);
}

#[test]
fn test_choose_multiple_random_selection() {
    let source = vec![1, 2, 3, 4, 5];
    let result = choose_multiple(source.clone(), 3);
    assert_eq!(result.len(), 3);
    for item in result.iter() {
        assert!(source.contains(item));
    }
}

