// Answer 0

#[test]
fn test_intersection_non_empty() {
    use hashbrown::HashSet;
    
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [4, 2, 3, 4].into_iter().collect();
    
    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, [2, 3].iter().collect::<HashSet<_>>());
}

#[test]
fn test_intersection_empty_a() {
    use hashbrown::HashSet;
    
    let a: HashSet<i32> = [].iter().cloned().collect();
    let b: HashSet<i32> = [4, 2, 3].iter().cloned().collect();
    
    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, [].iter().cloned().collect::<HashSet<i32>>());
}

#[test]
fn test_intersection_empty_b() {
    use hashbrown::HashSet;
    
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [].iter().cloned().collect();
    
    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, [].iter().cloned().collect::<HashSet<i32>>());
}

#[test]
fn test_intersection_no_common_elements() {
    use hashbrown::HashSet;
    
    let a: HashSet<i32> = [1, 2].iter().cloned().collect();
    let b: HashSet<i32> = [3, 4].iter().cloned().collect();
    
    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, [].iter().cloned().collect::<HashSet<i32>>());
}

#[test]
fn test_intersection_same_elements() {
    use hashbrown::HashSet;
    
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    
    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, [1, 2, 3].iter().cloned().collect::<HashSet<i32>>());
}

