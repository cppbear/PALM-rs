// Answer 0

#[test]
fn test_intersection_basic() {
    use crate::hashbrown::HashSet;

    let set_a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_b: HashSet<i32> = [4, 2, 3, 4].iter().cloned().collect();
    
    let intersection = set_a.intersection(&set_b);
    let result: HashSet<i32> = intersection.iter().cloned().collect();
    
    assert_eq!(result.len(), 2);
    assert!(result.contains(&2));
    assert!(result.contains(&3));
}

#[test]
fn test_intersection_empty_sets() {
    use crate::hashbrown::HashSet;

    let set_a: HashSet<i32> = HashSet::new();
    let set_b: HashSet<i32> = HashSet::new();
    
    let intersection = set_a.intersection(&set_b);
    let result: HashSet<i32> = intersection.iter().cloned().collect();
    
    assert!(result.is_empty());
}

#[test]
fn test_intersection_no_common_elements() {
    use crate::hashbrown::HashSet;

    let set_a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_b: HashSet<i32> = [4, 5, 6].iter().cloned().collect();
    
    let intersection = set_a.intersection(&set_b);
    let result: HashSet<i32> = intersection.iter().cloned().collect();
    
    assert!(result.is_empty());
}

#[test]
fn test_intersection_with_subsets() {
    use crate::hashbrown::HashSet;

    let set_a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_b: HashSet<i32> = [2, 3].iter().cloned().collect();
    
    let intersection = set_a.intersection(&set_b);
    let result: HashSet<i32> = intersection.iter().cloned().collect();
    
    assert_eq!(result.len(), 2);
    assert!(result.contains(&2));
    assert!(result.contains(&3));
}

