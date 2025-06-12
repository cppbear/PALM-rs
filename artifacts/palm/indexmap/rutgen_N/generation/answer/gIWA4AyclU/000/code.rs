// Answer 0

#[test]
fn test_append_with_non_empty_other() {
    use indexmap::IndexSet;

    let mut a = IndexSet::from([3, 2, 1]);
    let mut b = IndexSet::from([3, 4, 5]);
    let old_capacity = b.capacity();

    a.append(&mut b);

    assert_eq!(a.len(), 5);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
    assert!(a.iter().eq(&[3, 2, 1, 4, 5]));
}

#[test]
fn test_append_with_empty_other() {
    use indexmap::IndexSet;

    let mut a = IndexSet::from([1, 2, 3]);
    let mut b = IndexSet::new();

    a.append(&mut b);

    assert_eq!(a.len(), 3);
    assert_eq!(b.len(), 0);
    assert!(a.iter().eq(&[1, 2, 3]));
}

#[test]
fn test_append_self() {
    use indexmap::IndexSet;

    let mut a = IndexSet::from([1, 2, 3]);

    let old_capacity = a.capacity();
    a.append(&mut a.clone());

    assert_eq!(a.len(), 3);
    assert_eq!(a.capacity(), old_capacity);
    assert!(a.iter().eq(&[1, 2, 3]));
}

#[test]
fn test_append_large_sets() {
    use indexmap::IndexSet;

    let mut a = IndexSet::from_iter(1..=100);
    let mut b = IndexSet::from_iter(101..=200);
    let old_capacity = b.capacity();

    a.append(&mut b);

    assert_eq!(a.len(), 200);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
    assert!(a.iter().eq((1..=200).into_iter()));
}

