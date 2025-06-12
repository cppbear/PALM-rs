// Answer 0

#[test]
fn test_is_subset_empty_set() {
    use hashbrown::HashSet;
    
    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let set: HashSet<_> = HashSet::new();
    
    assert_eq!(set.is_subset(&sup), true);
}

#[test]
fn test_is_subset_with_common_elements() {
    use hashbrown::HashSet;
    
    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let mut set: HashSet<_> = HashSet::new();
    
    set.insert(2);
    assert_eq!(set.is_subset(&sup), true);
}

#[test]
fn test_is_subset_with_non_subset() {
    use hashbrown::HashSet;
    
    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let mut set: HashSet<_> = HashSet::new();
    
    set.insert(2);
    set.insert(4);
    assert_eq!(set.is_subset(&sup), false);
}

#[test]
fn test_is_subset_with_equal_sets() {
    use hashbrown::HashSet;
    
    let set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.is_subset(&set), true);
}

#[test]
fn test_is_subset_with_larger_set() {
    use hashbrown::HashSet;
    
    let sup: HashSet<_> = [1, 2, 3, 4, 5].into_iter().collect();
    let mut set: HashSet<_> = HashSet::new();
    
    set.insert(1);
    set.insert(3);
    set.insert(2);
    assert_eq!(set.is_subset(&sup), true);
}

#[test]
fn test_is_subset_with_set_longer_than_superset() {
    use hashbrown::HashSet;
    
    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let mut set: HashSet<_> = HashSet::new();
    
    set.insert(2);
    set.insert(4);
    set.insert(5);
    assert_eq!(set.is_subset(&sup), false);
    
    let mut long_set: HashSet<_> = HashSet::new();
    long_set.insert(1);
    long_set.insert(2);
    long_set.insert(3);
    long_set.insert(6);
    assert_eq!(long_set.is_subset(&sup), false);
}

