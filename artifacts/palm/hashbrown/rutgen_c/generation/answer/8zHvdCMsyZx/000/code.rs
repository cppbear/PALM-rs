// Answer 0

#[test]
fn test_is_subset_when_empty() {
    use hashbrown::HashSet;
    
    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let mut set = HashSet::new();
    
    assert_eq!(set.is_subset(&sup), true);
}

#[test]
fn test_is_subset_with_subset() {
    use hashbrown::HashSet;

    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let mut set = HashSet::new();
    
    set.insert(2);
    assert_eq!(set.is_subset(&sup), true);
}

#[test]
fn test_is_subset_with_non_subset() {
    use hashbrown::HashSet;

    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let mut set = HashSet::new();
    
    set.insert(4);
    assert_eq!(set.is_subset(&sup), false);
}

#[test]
fn test_is_subset_with_excess_elements() {
    use hashbrown::HashSet;

    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let mut set = HashSet::new();
    
    set.insert(1);
    set.insert(2);
    set.insert(4); // Element not in sup
    assert_eq!(set.is_subset(&sup), false);
}

#[test]
fn test_is_subset_when_equal() {
    use hashbrown::HashSet;

    let set: HashSet<_> = [1, 2, 3].into_iter().collect();
    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    
    assert_eq!(set.is_subset(&sup), true);
}

