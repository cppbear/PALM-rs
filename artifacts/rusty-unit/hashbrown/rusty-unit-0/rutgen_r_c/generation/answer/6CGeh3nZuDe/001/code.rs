// Answer 0

#[test]
fn test_union_with_non_empty_sets() {
    let set_a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let set_b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();
    
    let result_set = &set_a | &set_b;
    
    let expected: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result_vec: Vec<_> = result_set.iter().cloned().collect();
    assert!(expected.iter().all(|x| result_vec.contains(x)));
    assert_eq!(result_vec.len(), expected.len());
}

#[test]
fn test_union_with_empty_set() {
    let set_a: HashSet<i32> = vec![1, 2].into_iter().collect();
    let set_b: HashSet<i32> = HashSet::new(); // empty set
    
    let result_set = &set_a | &set_b;

    let expected: Vec<i32> = vec![1, 2];
    let result_vec: Vec<_> = result_set.iter().cloned().collect();
    assert!(expected.iter().all(|x| result_vec.contains(x)));
    assert_eq!(result_vec.len(), expected.len());
}

#[test]
fn test_union_with_identical_sets() {
    let set_a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let set_b: HashSet<i32> = set_a.clone();
    
    let result_set = &set_a | &set_b;
    
    let expected: Vec<i32> = vec![1, 2, 3];
    let result_vec: Vec<_> = result_set.iter().cloned().collect();
    assert!(expected.iter().all(|x| result_vec.contains(x)));
    assert_eq!(result_vec.len(), expected.len());
}

#[test]
fn test_union_with_disjoint_sets() {
    let set_a: HashSet<i32> = vec![1, 2].into_iter().collect();
    let set_b: HashSet<i32> = vec![3, 4].into_iter().collect();
    
    let result_set = &set_a | &set_b;

    let expected: Vec<i32> = vec![1, 2, 3, 4];
    let result_vec: Vec<_> = result_set.iter().cloned().collect();
    assert!(expected.iter().all(|x| result_vec.contains(x)));
    assert_eq!(result_vec.len(), expected.len());
}

#[test]
fn test_union_empty_sets() {
    let set_a: HashSet<i32> = HashSet::new(); // empty set
    let set_b: HashSet<i32> = HashSet::new(); // empty set
    
    let result_set = &set_a | &set_b;

    let expected: Vec<i32> = vec![];
    let result_vec: Vec<_> = result_set.iter().cloned().collect();
    assert_eq!(result_vec, expected);
}

