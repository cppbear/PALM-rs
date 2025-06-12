// Answer 0

#[test]
fn test_take_existing_value() {
    use hashbrown::HashSet;

    let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.take(&2), Some(2));
    assert_eq!(set.take(&2), None);
}

#[test]
fn test_take_non_existing_value() {
    use hashbrown::HashSet;

    let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.take(&4), None);
}

#[test]
fn test_take_multiple_instances() {
    use hashbrown::HashSet;

    let mut set: HashSet<_> = vec![1, 1, 2, 3].into_iter().collect(); // HashSet handles duplicates
    assert_eq!(set.take(&1), Some(1));
    assert_eq!(set.take(&1), None);
}

