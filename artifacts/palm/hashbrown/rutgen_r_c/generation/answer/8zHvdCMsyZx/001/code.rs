// Answer 0

#[test]
fn test_is_subset_with_equal_sets() {
    let set_a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    assert_eq!(set_a.is_subset(&set_b), true);
}

#[test]
fn test_is_subset_with_empty_subset() {
    let set_a: HashSet<i32> = HashSet::new();
    let set_b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    assert_eq!(set_a.is_subset(&set_b), true);
}

#[test]
fn test_is_subset_with_equal_size_non_subset() {
    let set_a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_b: HashSet<i32> = [2, 3, 4].iter().cloned().collect(); // Different elements
    assert_eq!(set_a.is_subset(&set_b), false);
}

#[test]
fn test_is_subset_with_single_element_subset() {
    let set_a: HashSet<i32> = [2].iter().cloned().collect();
    let set_b: HashSet<i32> = [1, 2].iter().cloned().collect();
    assert_eq!(set_a.is_subset(&set_b), true);
}

#[test]
fn test_is_subset_with_identical_large_sets() {
    let set_a: HashSet<i32> = (0..1000).collect();
    let set_b: HashSet<i32> = (0..1000).collect();
    assert_eq!(set_a.is_subset(&set_b), true);
}

#[test]
fn test_is_subset_with_intersecting_elements() {
    let set_a: HashSet<i32> = [1, 2].iter().cloned().collect();
    let set_b: HashSet<i32> = [2, 3].iter().cloned().collect();
    assert_eq!(set_a.is_subset(&set_b), false);
}

