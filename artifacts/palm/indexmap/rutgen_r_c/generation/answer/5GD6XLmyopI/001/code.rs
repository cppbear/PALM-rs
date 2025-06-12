// Answer 0

#[test]
fn test_append_non_empty_to_empty() {
    let mut a = IndexMap::from([(1, "a"), (2, "b")]);
    let mut b = IndexMap::from([(3, "c"), (4, "d")]);
    let old_capacity = b.capacity();

    a.append(&mut b);

    assert_eq!(a.len(), 4);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
    assert!(a.keys().eq(&[1, 2, 3, 4]));
}

#[test]
fn test_append_empty_to_non_empty() {
    let mut a = IndexMap::from([(1, "a"), (2, "b")]);
    let mut b = IndexMap::new();
    let old_capacity = b.capacity();

    a.append(&mut b);

    assert_eq!(a.len(), 2);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
    assert!(a.keys().eq(&[1, 2]));
}

#[test]
fn test_append_non_empty_to_non_empty_with_key_collision() {
    let mut a = IndexMap::from([(1, "a"), (2, "b"), (3, "c")]);
    let mut b = IndexMap::from([(3, "d"), (4, "e")]);
    let old_capacity = b.capacity();

    a.append(&mut b);

    assert_eq!(a.len(), 4);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
    assert!(a.keys().eq(&[1, 2, 3, 4]));
    assert_eq!(a[&3], "d"); // "c" was overwritten.
}

#[test]
fn test_append_large_maps() {
    let mut a = IndexMap::from_iter((0..1000).map(|x| (x, x.to_string())));
    let mut b = IndexMap::from_iter((1000..2000).map(|x| (x, x.to_string())));
    let old_capacity = b.capacity();

    a.append(&mut b);

    assert_eq!(a.len(), 2000);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
    assert!(a.keys().eq((0..2000).map(|x| x)));
}

#[should_panic]
fn test_append_panic_on_mutably_borrowing() {
    let mut a = IndexMap::new();
    let mut b = IndexMap::new();

    // Creating two mutable references to the same IndexMap
    let a_ref = &mut a;
    let b_ref = &mut b; 

    // Should panic because we're trying to append while a mutable borrow exists
    a_ref.append(b_ref);
}

