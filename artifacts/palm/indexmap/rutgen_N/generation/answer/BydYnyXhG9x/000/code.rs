// Answer 0

#[test]
fn test_retain_keep_some_elements() {
    use indexmap::IndexMap;
    
    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.insert(1, ());
    set.insert(2, ());
    set.insert(3, ());
    
    set.retain(|&x| x % 2 == 0);
    
    assert_eq!(set.keys().copied().collect::<Vec<_>>(), vec![2]);
}

#[test]
fn test_retain_keep_no_elements() {
    use indexmap::IndexMap;
    
    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.insert(1, ());
    set.insert(2, ());
    set.insert(3, ());
    
    set.retain(|_| false);
    
    assert!(set.is_empty());
}

#[test]
fn test_retain_keep_all_elements() {
    use indexmap::IndexMap;
    
    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.insert(1, ());
    set.insert(2, ());
    set.insert(3, ());
    
    set.retain(|_| true);
    
    assert_eq!(set.keys().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
}

#[test]
fn test_retain_empty_set() {
    use indexmap::IndexMap;
    
    let mut set: IndexMap<i32, ()> = IndexMap::new();
    
    set.retain(|_| true);
    
    assert!(set.is_empty());
}

