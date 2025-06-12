// Answer 0

#[test]
fn test_append_empty_other() {
    let mut a = IndexMap::from([(1, "a"), (2, "b")]);
    let mut b: IndexMap<_, _> = IndexMap::new();
    
    a.append(&mut b);
    
    assert_eq!(a.len(), 2);
    assert_eq!(b.len(), 0);
}

#[test]
fn test_append_non_empty_other() {
    let mut a = IndexMap::from([(1, "a"), (2, "b")]);
    let mut b = IndexMap::from([(3, "c"), (4, "d")]);
    
    a.append(&mut b);
    
    assert_eq!(a.len(), 4);
    assert_eq!(b.len(), 0);
    assert_eq!(a[&3], "c");
    assert_eq!(a[&4], "d");
}

#[test]
fn test_append_with_overwriting() {
    let mut a = IndexMap::from([(1, "a"), (2, "b"), (3, "c")]);
    let mut b = IndexMap::from([(3, "d"), (4, "e")]);
    
    a.append(&mut b);
    
    assert_eq!(a.len(), 4);
    assert_eq!(b.len(), 0);
    assert_eq!(a[&3], "d"); // "c" should be overwritten by "d"
    assert_eq!(a[&4], "e");
}

#[test]
fn test_append_with_capacity_check() {
    let mut a = IndexMap::from([(1, "a")]);
    let mut b = IndexMap::from([(2, "b"), (3, "c"), (4, "d")]);
    let old_capacity = b.capacity();
    
    a.append(&mut b);
    
    assert_eq!(a.len(), 4);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
}

