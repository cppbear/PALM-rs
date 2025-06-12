// Answer 0

#[test]
fn test_is_disjoint_with_empty_sets() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();

    assert_eq!(a.is_disjoint(&b), true);
}

#[test]
fn test_is_disjoint_with_non_empty_and_empty_set() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    assert_eq!(a.is_disjoint(&b), true);
}

#[test]
fn test_is_disjoint_with_disjoint_non_empty_sets() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(4);

    assert_eq!(a.is_disjoint(&b), true);
}

#[test]
fn test_is_disjoint_with_common_element() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(1);

    assert_eq!(a.is_disjoint(&b), false);
}

#[test]
fn test_is_disjoint_with_multiple_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(1);
    b.insert(2);

    assert_eq!(a.is_disjoint(&b), false);
}

#[test]
fn test_is_disjoint_with_large_sets() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = (0..1000).collect();
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(1000);

    assert_eq!(a.is_disjoint(&b), true);
}

#[test]
fn test_is_disjoint_with_one_common_and_one_missing() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(2);
    
    assert_eq!(a.is_disjoint(&b), false);
}

