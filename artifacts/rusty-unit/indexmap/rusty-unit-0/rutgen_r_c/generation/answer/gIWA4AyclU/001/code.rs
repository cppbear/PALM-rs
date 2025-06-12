// Answer 0

#[test]
fn test_append_non_empty_to_empty() {
    let mut a = IndexSet::from([1, 2, 3]);
    let mut b = IndexSet::from([]);
    
    a.append(&mut b);
    
    assert_eq!(a.len(), 3);
    assert_eq!(b.len(), 0);
    assert!(a.iter().eq(&[1, 2, 3]));
}

#[test]
fn test_append_empty_to_non_empty() {
    let mut a = IndexSet::from([]);
    let mut b = IndexSet::from([4, 5, 6]);
    
    a.append(&mut b);
    
    assert_eq!(a.len(), 3);
    assert_eq!(b.len(), 0);
    assert!(a.iter().eq(&[4, 5, 6]));
}

#[test]
fn test_append_non_empty_to_non_empty() {
    let mut a = IndexSet::from([1, 2, 3]);
    let mut b = IndexSet::from([3, 4, 5]);
    
    let old_capacity = b.capacity();
    a.append(&mut b);
    
    assert_eq!(a.len(), 5);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
    assert!(a.iter().eq(&[1, 2, 3, 3, 4, 5]));
}

#[test]
fn test_append_with_duplicates() {
    let mut a = IndexSet::from([1, 2, 3]);
    let mut b = IndexSet::from([2, 3, 4]);
    
    let old_capacity = b.capacity();
    a.append(&mut b);
    
    assert_eq!(a.len(), 5);
    assert_eq!(b.len(), 0);
    assert_eq!(b.capacity(), old_capacity);
    assert!(a.iter().eq(&[1, 2, 3, 2, 3, 4]));
}

