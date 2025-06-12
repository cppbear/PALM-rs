// Answer 0

#[test]
fn test_get_existing_value() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.get(&2), Some(&2));
}

#[test]
fn test_get_non_existing_value() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.get(&4), None);
}

#[test]
fn test_get_with_zero() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = [0, 1, 2, 3].into_iter().collect();
    assert_eq!(set.get(&0), Some(&0));
}

#[test]
fn test_get_with_negative_number() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = [-1, 0, 1].into_iter().collect();
    assert_eq!(set.get(&-1), Some(&-1));
}

#[test]
fn test_get_on_empty_set() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = HashSet::new();
    assert_eq!(set.get(&1), None);
}

