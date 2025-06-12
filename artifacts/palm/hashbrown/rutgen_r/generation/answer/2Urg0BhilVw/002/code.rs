// Answer 0

#[test]
fn test_intersection_larger_self() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3, 4, 5].into_iter().collect();
    let b: HashSet<_> = [2, 3].into_iter().collect();

    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, b);
}

#[test]
fn test_intersection_equal_size() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [2, 3].into_iter().collect();

    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, b);
}

#[test]
fn test_intersection_no_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [4, 5, 6].into_iter().collect();

    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection.len(), 0);
}

#[test]
fn test_intersection_empty_self() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<_> = [1, 2, 3].into_iter().collect();

    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection.len(), 0);
}

#[test]
fn test_intersection_empty_other() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection.len(), 0);
}

