// Answer 0

#[test]
fn test_new_with_non_empty_sets() {
    use indexmap::IndexSet;

    let set: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    let other: IndexSet<i32> = IndexSet::from_iter(vec![4, 5]);

    let result = new(&set, &other);
    
    assert_eq!(result.iter.clone().collect::<Vec<_>>(), vec![1, 2, 3]);
    assert_eq!(result.other.len(), 2);
}

#[test]
fn test_new_with_empty_first_set() {
    use indexmap::IndexSet;

    let set: IndexSet<i32> = IndexSet::new();
    let other: IndexSet<i32> = IndexSet::from_iter(vec![4, 5]);

    let result = new(&set, &other);

    assert_eq!(result.iter.clone().collect::<Vec<_>>(), Vec::<i32>::new());
    assert_eq!(result.other.len(), 2);
}

#[test]
fn test_new_with_empty_second_set() {
    use indexmap::IndexSet;

    let set: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    let other: IndexSet<i32> = IndexSet::new();

    let result = new(&set, &other);

    assert_eq!(result.iter.clone().collect::<Vec<_>>(), vec![1, 2, 3]);
    assert_eq!(result.other.len(), 0);
}

#[test]
fn test_new_with_identical_sets() {
    use indexmap::IndexSet;

    let set: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    let other: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);

    let result = new(&set, &other);
    
    assert_eq!(result.iter.clone().collect::<Vec<_>>(), vec![1, 2, 3]);
    assert_eq!(result.other.clone().collect::<Vec<_>>(), vec![1, 2, 3]);
}

#[test]
fn test_new_with_different_types() {
    use indexmap::IndexSet;

    let set: IndexSet<String> = IndexSet::from_iter(vec!["a".to_string(), "b".to_string()]);
    let other: IndexSet<String> = IndexSet::from_iter(vec!["c".to_string()]);

    let result = new(&set, &other);

    assert_eq!(result.iter.clone().collect::<Vec<_>>(), vec!["a", "b"]);
    assert_eq!(result.other.len(), 1);
}

