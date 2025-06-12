// Answer 0

#[test]
fn test_is_disjoint_empty_sets() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();

    assert_eq!(a.is_disjoint(&b), true);
}

#[test]
fn test_is_disjoint_one_empty_set() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    assert_eq!(a.is_disjoint(&b), true);
    assert_eq!(b.is_disjoint(&a), true);
}

#[test]
fn test_is_disjoint_no_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(4);

    assert_eq!(a.is_disjoint(&b), true);
    assert_eq!(b.is_disjoint(&a), true);
}

#[test]
fn test_is_disjoint_with_common_element() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(1);

    assert_eq!(a.is_disjoint(&b), false);
    assert_eq!(b.is_disjoint(&a), false);
}

#[test]
fn test_is_disjoint_with_multiple_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(2);
    b.insert(5);

    assert_eq!(a.is_disjoint(&b), false);
    assert_eq!(b.is_disjoint(&a), false);
}

