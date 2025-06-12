// Answer 0

#[test]
fn test_is_superset_empty_sets() {
    let set_a: HashSet<i32> = HashSet::new();
    let set_b: HashSet<i32> = HashSet::new();
    
    assert_eq!(set_a.is_superset(&set_b), true);
}

#[test]
fn test_is_superset_with_one_set_empty() {
    let set_a: HashSet<i32> = HashSet::new();
    let set_b: HashSet<i32> = [1].iter().cloned().collect();
    
    assert_eq!(set_a.is_superset(&set_b), false);
}

#[test]
fn test_is_superset_with_unequal_sets() {
    let set_a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_b: HashSet<i32> = [1, 2].iter().cloned().collect();
    
    assert_eq!(set_a.is_superset(&set_b), true);
}

#[test]
fn test_is_superset_with_subset() {
    let set_a: HashSet<i32> = [1, 2].iter().cloned().collect();
    let set_b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    
    assert_eq!(set_a.is_superset(&set_b), false);
}

#[test]
fn test_is_superset_with_disjoint_sets() {
    let set_a: HashSet<i32> = [1, 2].iter().cloned().collect();
    let set_b: HashSet<i32> = [3, 4].iter().cloned().collect();
    
    assert_eq!(set_a.is_superset(&set_b), false);
}

#[test]
fn test_is_superset_with_same_sets() {
    let set_a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    
    assert_eq!(set_a.is_superset(&set_b), true);
}

#[test]
fn test_is_superset_with_extra_element() {
    let set_a: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set_b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    
    assert_eq!(set_a.is_superset(&set_b), true);
}

#[test]
fn test_is_superset_multiple_elements() {
    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<i32> = [3, 4, 5].iter().cloned().collect();
    
    assert_eq!(set_a.is_superset(&set_b), true);
}

