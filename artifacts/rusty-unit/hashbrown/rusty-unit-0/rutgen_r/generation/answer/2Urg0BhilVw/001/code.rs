// Answer 0

#[test]
fn test_intersection_equal_size() {
    use hashbrown::HashSet;
    
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [2, 3, 4].into_iter().collect();

    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, [2, 3].iter().collect::<HashSet<_>>());
}

#[test]
fn test_intersection_empty_sets() {
    use hashbrown::HashSet;
    
    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();

    let intersection: HashSet<i32> = a.intersection(&b).collect();
    assert_eq!(intersection, HashSet::new());
}

#[test]
fn test_intersection_no_common_elements() {
    use hashbrown::HashSet;
    
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [4, 5, 6].into_iter().collect();

    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, HashSet::new());
}

#[test]
fn test_intersection_all_common_elements() {
    use hashbrown::HashSet;
    
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [1, 2, 3].into_iter().collect();

    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, a);
}

#[test]
fn test_intersection_different_sizes() {
    use hashbrown::HashSet;
    
    let a: HashSet<_> = [1, 2].into_iter().collect();
    let b: HashSet<_> = [1, 2, 3, 4].into_iter().collect();

    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, a);
}

