// Answer 0

#[test]
fn test_difference_non_empty_sets() {
    use hashbrown::HashSet;
    
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [4, 2, 3, 4].iter().cloned().collect();
    
    let diff: HashSet<i32> = a.difference(&b).collect();
    assert_eq!(diff, [1].iter().cloned().collect());
}

#[test]
fn test_difference_empty_and_non_empty_set() {
    use hashbrown::HashSet;
    
    let a: HashSet<i32> = [].iter().cloned().collect();
    let b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    
    let diff: HashSet<i32> = a.difference(&b).collect();
    assert_eq!(diff, [].iter().cloned().collect());
}

#[test]
fn test_difference_non_empty_and_empty_set() {
    use hashbrown::HashSet;
    
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [].iter().cloned().collect();
    
    let diff: HashSet<i32> = a.difference(&b).collect();
    assert_eq!(diff, [1, 2, 3].iter().cloned().collect());
}

#[test]
fn test_difference_identical_sets() {
    use hashbrown::HashSet;
    
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    
    let diff: HashSet<i32> = a.difference(&b).collect();
    assert_eq!(diff, [].iter().cloned().collect());
}

#[test]
fn test_difference_with_disjoint_sets() {
    use hashbrown::HashSet;
    
    let a: HashSet<i32> = [1, 2].iter().cloned().collect();
    let b: HashSet<i32> = [3, 4].iter().cloned().collect();
    
    let diff: HashSet<i32> = a.difference(&b).collect();
    assert_eq!(diff, [1, 2].iter().cloned().collect());
}

