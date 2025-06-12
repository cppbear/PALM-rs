// Answer 0

#[test]
fn test_symmetric_difference_debug_non_empty() {
    use std::collections::hash_set::HashSet;

    let a: HashSet<i32> = HashSet::from_iter(vec![1, 2, 3, 4]);
    let b: HashSet<i32> = HashSet::from_iter(vec![3, 4, 5, 6]);
    
    let symmetric_difference = SymmetricDifference {
        iter: a.difference(&b).chain(b.difference(&a)),
    };

    let _ = format!("{:?}", symmetric_difference);
}

#[test]
fn test_symmetric_difference_debug_empty() {
    use std::collections::hash_set::HashSet;

    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();
    
    let symmetric_difference = SymmetricDifference {
        iter: a.difference(&b).chain(b.difference(&a)),
    };

    let _ = format!("{:?}", symmetric_difference);
}

#[test]
fn test_symmetric_difference_debug_single_element() {
    use std::collections::hash_set::HashSet;

    let a: HashSet<i32> = HashSet::from_iter(vec![2]);
    let b: HashSet<i32> = HashSet::from_iter(vec![3]);
    
    let symmetric_difference = SymmetricDifference {
        iter: a.difference(&b).chain(b.difference(&a)),
    };

    let _ = format!("{:?}", symmetric_difference);
}

#[test]
fn test_symmetric_difference_debug_large_sets() {
    use std::collections::hash_set::HashSet;

    let a: HashSet<i32> = (0..512).collect();
    let b: HashSet<i32> = (256..768).collect();
    
    let symmetric_difference = SymmetricDifference {
        iter: a.difference(&b).chain(b.difference(&a)),
    };

    let _ = format!("{:?}", symmetric_difference);
}

