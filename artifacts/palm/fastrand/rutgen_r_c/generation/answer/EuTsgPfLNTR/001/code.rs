// Answer 0

#[test]
fn test_choose_multiple_with_non_empty_source() {
    let source = vec![1, 2, 3, 4, 5];
    let amount = 3;
    let result = choose_multiple(source, amount);
    assert_eq!(result.len(), amount);
    for &item in &result {
        assert!(source.contains(&item));
    }
}

#[test]
fn test_choose_multiple_with_amount_zero() {
    let source = vec![1, 2, 3, 4, 5];
    let amount = 0;
    let result = choose_multiple(source, amount);
    assert_eq!(result.len(), 0);
}

#[should_panic]
fn test_choose_multiple_with_empty_source() {
    let source: Vec<u32> = vec![];
    let amount = 1;
    choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_large_amount() {
    let source = vec![1, 2, 3, 4, 5];
    let amount = 10; // More than the size of the source
    let result = choose_multiple(source.clone(), amount);
    assert!(result.len() <= source.len());
    for &item in &result {
        assert!(source.contains(&item));
    }
}

