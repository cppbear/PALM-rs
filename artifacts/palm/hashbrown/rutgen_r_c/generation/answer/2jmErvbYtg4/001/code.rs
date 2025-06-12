// Answer 0

#[test]
fn test_clear_non_empty_set() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    assert!(!set.is_empty());
    set.clear();
    assert!(set.is_empty());
}

#[test]
fn test_clear_empty_set() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    
    assert!(set.is_empty());
    set.clear();
    assert!(set.is_empty());
}

#[test]
fn test_clear_set_with_multiple_elements() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    set.insert(4);
    set.insert(5);
    set.insert(6);
    set.insert(7);
    
    assert_eq!(set.len(), 4);
    set.clear();
    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
}

#[test]
fn test_clear_large_set() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    for i in 0..1000 {
        set.insert(i);
    }
    
    assert_eq!(set.len(), 1000);
    set.clear();
    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
}

