// Answer 0

#[test]
fn test_get_existing_value() {
    use hashbrown::HashSet;

    let set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.get(&2), Some(&2));
}

#[test]
fn test_get_non_existing_value() {
    use hashbrown::HashSet;

    let set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.get(&4), None);
}

#[test]
fn test_get_with_borrowed_value() {
    use hashbrown::HashSet;

    let set: HashSet<_> = [1, 2, 3].into_iter().collect();
    let value: &i32 = &3;
    assert_eq!(set.get(value), Some(&3));
}

#[test]
fn test_get_with_different_reference() {
    use hashbrown::HashSet;

    let set: HashSet<_> = [1, 2, 3].into_iter().collect();
    let value: &i32 = &1;
    assert_eq!(set.get(value), Some(&1));
}

#[test]
fn test_get_empty_set() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = HashSet::new();
    assert_eq!(set.get(&1), None);
}

