// Answer 0

#[test]
fn test_contains_value_present() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.contains(&1), true);
}

#[test]
fn test_contains_value_absent() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.contains(&4), false);
}

#[test]
fn test_contains_with_negative_values() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = [-1, -2, -3].into_iter().collect();
    assert_eq!(set.contains(&-1), true);
    assert_eq!(set.contains(&0), false);
}

#[test]
fn test_contains_with_edge_case() {
    use hashbrown::HashSet;

    let set: HashSet<u32> = [u32::MAX, u32::MIN].into_iter().collect();
    assert_eq!(set.contains(&u32::MAX), true);
    assert_eq!(set.contains(&u32::MIN), true);
    assert_eq!(set.contains(&(u32::MAX + 1)), false);
}

