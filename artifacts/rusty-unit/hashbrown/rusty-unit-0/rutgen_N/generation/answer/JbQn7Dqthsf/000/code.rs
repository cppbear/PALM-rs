// Answer 0

#[test]
fn test_get_or_insert_existing_value() {
    use hashbrown::HashSet;

    let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.len(), 3);
    assert_eq!(set.get_or_insert(2), &2);
    assert_eq!(set.len(), 3); // 2 already exists, so length should not change
}

#[test]
fn test_get_or_insert_new_value() {
    use hashbrown::HashSet;

    let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.len(), 3);
    assert_eq!(set.get_or_insert(100), &100);
    assert_eq!(set.len(), 4); // 100 was inserted, so length should increase
}

#[test]
fn test_get_or_insert_multiple_new_values() {
    use hashbrown::HashSet;

    let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.len(), 3);
    assert_eq!(set.get_or_insert(4), &4);
    assert_eq!(set.get_or_insert(5), &5);
    assert_eq!(set.len(), 5); // Two new values (4 and 5) were inserted
}

#[test]
fn test_get_or_insert_same_value() {
    use hashbrown::HashSet;

    let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.len(), 3);
    assert_eq!(set.get_or_insert(2), &2);
    assert_eq!(set.get_or_insert(2), &2); // Inserting the same value again
    assert_eq!(set.len(), 3); // Length should remain the same
}

