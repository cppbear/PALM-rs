// Answer 0

#[test]
fn test_intersection_equal_length() {
    use hashbrown::HashSet;
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [2, 3].into_iter().collect();

    let intersection = a.intersection(&b);
    let results: Vec<_> = intersection.iter.collect();

    assert_eq!(results, vec![2, 3]);
}

#[test]
fn test_intersection_empty_sets() {
    use hashbrown::HashSet;
    let a: HashSet<_> = [].into_iter().collect();
    let b: HashSet<_> = [].into_iter().collect();

    let intersection = a.intersection(&b);
    let results: Vec<_> = intersection.iter.collect();

    assert_eq!(results, Vec::<i32>::new());
}

#[test]
fn test_intersection_same_elements() {
    use hashbrown::HashSet;
    let a: HashSet<_> = [1, 1, 1].into_iter().collect();
    let b: HashSet<_> = [1, 1, 1].into_iter().collect();

    let intersection = a.intersection(&b);
    let results: Vec<_> = intersection.iter.collect();

    assert_eq!(results, vec![1]);
}

