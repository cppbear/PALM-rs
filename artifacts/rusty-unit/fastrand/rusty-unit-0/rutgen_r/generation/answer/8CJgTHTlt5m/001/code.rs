// Answer 0

#[test]
fn test_choice_empty_iterator() {
    let empty: Vec<i32> = Vec::new();
    let result = choice(empty.into_iter());
    assert_eq!(result, None);
}

#[test]
fn test_choice_single_item() {
    let single_item = vec![42];
    let result = choice(single_item.into_iter());
    assert_eq!(result, Some(42));
}

#[test]
fn test_choice_multiple_items() {
    let multiple_items = vec![1, 2, 3, 4, 5];
    let result = choice(multiple_items.into_iter());
    assert!(result.is_some());
    assert!(result.unwrap() >= 1 && result.unwrap() <= 5);
}

#[test]
fn test_choice_large_iterator() {
    let large_items: Vec<i32> = (1..=1000).collect();
    let result = choice(large_items.into_iter());
    assert!(result.is_some());
    assert!(result.unwrap() >= 1 && result.unwrap() <= 1000);
}

