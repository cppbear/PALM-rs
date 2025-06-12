// Answer 0

#[test]
fn test_append_with_non_empty_maps() {
    use indexmap::IndexMap;

    let mut a = IndexMap::from([(3, "c"), (2, "b"), (1, "a")]);
    let mut b = IndexMap::from([(3, "d"), (4, "e"), (5, "f")]);
    let old_capacity = b.capacity();

    a.append(&mut b);

    assert_eq!(a.len(), 5);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
    assert!(a.keys().eq(&[3, 2, 1, 4, 5]));
    assert_eq!(a[&3], "d");
}

#[test]
fn test_append_with_empty_other() {
    use indexmap::IndexMap;

    let mut a = IndexMap::from([(1, "a"), (2, "b")]);
    let mut b = IndexMap::<i32, &str>::new();

    let old_capacity = b.capacity();
    
    a.append(&mut b);

    assert_eq!(a.len(), 2);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
}

#[test]
fn test_append_with_same_keys() {
    use indexmap::IndexMap;

    let mut a = IndexMap::from([(1, "a"), (2, "b")]);
    let mut b = IndexMap::from([(1, "c"), (3, "d")]);

    let old_capacity = b.capacity();

    a.append(&mut b);

    assert_eq!(a.len(), 3);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
    assert_eq!(a[&1], "c");
}

#[test]
fn test_append_with_large_maps() {
    use indexmap::IndexMap;

    let mut a = IndexMap::from_iter((1..1000).map(|i| (i, i.to_string())));
    let mut b = IndexMap::from_iter((1000..2000).map(|i| (i, i.to_string())));
    
    let old_capacity = b.capacity();

    a.append(&mut b);

    assert_eq!(a.len(), 2000);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
}

#[test]
#[should_panic]
fn test_append_should_panic_if_b_referenced_after_append() {
    use indexmap::IndexMap;

    let mut a = IndexMap::from([(1, "a")]);
    let mut b = IndexMap::from([(2, "b")]);

    a.append(&mut b);

    // This panic is expected, as `b` should be empty after append
    assert!(b.get(&2).is_some());
}

