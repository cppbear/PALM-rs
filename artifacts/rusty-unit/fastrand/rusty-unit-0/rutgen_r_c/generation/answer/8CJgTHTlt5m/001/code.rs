// Answer 0

#[test]
fn test_choice_non_empty_iterator() {
    let items = vec![1, 2, 3, 4, 5];
    let result = choice(items);
    assert!(result.is_some());
    let chosen_item = result.unwrap();
    assert!(items.contains(&chosen_item));
}

#[test]
fn test_choice_empty_iterator() {
    let items: Vec<i32> = Vec::new();
    let result = choice(items);
    assert!(result.is_none());
}

#[test]
fn test_choice_single_item_iterator() {
    let items = vec![42];
    let result = choice(items);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_choice_large_iterator() {
    let items: Vec<i32> = (1..=1000).collect();
    let result = choice(items);
    assert!(result.is_some());
    let chosen_item = result.unwrap();
    assert!(items.contains(&chosen_item));
}

#[should_panic(expect = "Panics if the range is empty.")]
#[test]
fn test_choice_invalid_iterator() {
    let empty_items: std::iter::Empty<i32> = std::iter::empty();
    let _result = choice(empty_items);
}

