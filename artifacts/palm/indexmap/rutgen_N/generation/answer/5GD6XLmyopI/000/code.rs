// Answer 0

#[test]
fn test_append_key_updates_existing_value() {
    use indexmap::IndexMap;

    let mut a = IndexMap::from([(3, "c"), (2, "b"), (1, "a")]);
    let mut b = IndexMap::from([(3, "d"), (4, "e"), (5, "f")]);
    let old_capacity = b.capacity();

    a.append(&mut b);

    assert_eq!(a.len(), 5);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);

    assert!(a.keys().eq(&[3, 2, 1, 4, 5]));
    assert_eq!(a[&3], "d"); // "c" was overwritten.
}

#[test]
fn test_append_empty_other() {
    use indexmap::IndexMap;

    let mut a = IndexMap::from([(1, "a"), (2, "b")]);
    let mut b = IndexMap::new();

    a.append(&mut b);

    assert_eq!(a.len(), 2);
    assert_eq!(b.len(), 0);
}

#[test]
fn test_append_other_empty() {
    use indexmap::IndexMap;

    let mut a = IndexMap::new();
    let mut b = IndexMap::from([(1, "a"), (2, "b")]);

    a.append(&mut b);

    assert_eq!(a.len(), 2);
    assert_eq!(b.len(), 0);
}

#[test]
fn test_append_non_overlapping_keys() {
    use indexmap::IndexMap;

    let mut a = IndexMap::from([(1, "a"), (2, "b")]);
    let mut b = IndexMap::from([(3, "c"), (4, "d")]);
    let old_capacity = b.capacity();

    a.append(&mut b);

    assert_eq!(a.len(), 4);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);

    assert!(a.keys().eq(&[1, 2, 3, 4]));
}

