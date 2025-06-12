// Answer 0

#[test]
fn test_append_non_empty_indexset() {
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
fn test_append_empty_indexset() {
    use indexmap::IndexSet;
    
    let mut a = IndexSet::from([1, 2, 3]);
    let mut b = IndexSet::new(); // empty set

    a.append(&mut b);

    assert_eq!(a.len(), 3);
    assert_eq!(b.len(), 0);
    assert!(a.iter().eq(&[1, 2, 3]));
}

#[test]
fn test_append_with_duplicates() {
    use indexmap::IndexSet;
    
    let mut a = IndexSet::from([1, 2, 3]);
    let mut b = IndexSet::from([2, 3, 4]); // contains duplicates with `a`

    a.append(&mut b);

    assert_eq!(a.len(), 4); // 1, 2, 3, 4
    assert_eq!(b.len(), 0);
    assert!(a.iter().eq(&[1, 2, 3, 4]));
}

#[test]
fn test_append_large_indexsets() {
    use indexmap::IndexSet;
    
    let mut a = IndexSet::from_iter(1..=1000);
    let mut b = IndexSet::from_iter(1001..=2000);

    let old_capacity = b.capacity();
    a.append(&mut b);

    assert_eq!(a.len(), 2000);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
    assert!(a.iter().eq(1..=2000));
}

#[test]
fn test_append_self() {
    use indexmap::IndexSet;

    let mut a = IndexSet::from([1, 2, 3]);

    a.append(&mut a);

    assert_eq!(a.len(), 3);
    assert!(a.iter().eq(&[1, 2, 3]));
}

