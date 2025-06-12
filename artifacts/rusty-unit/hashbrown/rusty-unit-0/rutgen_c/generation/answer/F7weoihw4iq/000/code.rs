// Answer 0

#[test]
fn test_is_superset_with_non_empty_subset() {
    use hashbrown::HashSet;
    
    let sub: HashSet<_> = [1, 2].into_iter().collect();
    let mut set = HashSet::new();
    
    assert_eq!(set.is_superset(&sub), false);
    
    set.insert(0);
    set.insert(1);
    assert_eq!(set.is_superset(&sub), false);
    
    set.insert(2);
    assert_eq!(set.is_superset(&sub), true);
}

#[test]
fn test_is_superset_with_empty_subset() {
    use hashbrown::HashSet;
    
    let empty_set: HashSet<i32> = HashSet::new();
    let mut set = HashSet::new();

    assert_eq!(set.is_superset(&empty_set), true);

    set.insert(1);
    assert_eq!(set.is_superset(&empty_set), true);
}

#[test]
fn test_is_superset_when_equal() {
    use hashbrown::HashSet;
    
    let mut set1: HashSet<_> = [1, 2].into_iter().collect();
    let set2 = set1.clone();

    assert_eq!(set1.is_superset(&set2), true);
}

#[test]
fn test_is_superset_with_no_elements() {
    use hashbrown::HashSet;
    
    let empty_set1: HashSet<i32> = HashSet::new();
    let empty_set2: HashSet<i32> = HashSet::new();
    
    assert_eq!(empty_set1.is_superset(&empty_set2), true);
}

#[test]
fn test_is_superset_with_large_subset() {
    use hashbrown::HashSet;
    
    let sub: HashSet<_> = (1..=100).into_iter().collect();
    let mut set = HashSet::new();
    
    for i in 1..=150 {
        set.insert(i);
    }

    assert_eq!(set.is_superset(&sub), true);
}

