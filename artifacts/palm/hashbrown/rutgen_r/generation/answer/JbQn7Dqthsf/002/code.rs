// Answer 0

#[test]
fn test_get_or_insert_existing_value() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.len(), 3);
    assert_eq!(set.get_or_insert(2), &2);
    assert_eq!(set.len(), 3); // 2 already exists, set length should not change
}

#[test]
fn test_get_or_insert_new_value() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.len(), 3);
    assert_eq!(set.get_or_insert(100), &100);
    assert_eq!(set.len(), 4); // 100 was inserted
}

#[test]
fn test_get_or_insert_multiple_new_values() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = [5, 6, 7].into_iter().collect();
    assert_eq!(set.len(), 3);
    assert_eq!(set.get_or_insert(8), &8);
    assert_eq!(set.get_or_insert(9), &9);
    assert_eq!(set.len(), 5); // 8 and 9 were inserted
}

#[test]
fn test_get_or_insert_with_zero() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = [0, -1, 1].into_iter().collect();
    assert_eq!(set.len(), 3);
    assert_eq!(set.get_or_insert(0), &0); // 0 already exists
    assert_eq!(set.len(), 3); // Length should not change
}

