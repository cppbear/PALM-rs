// Answer 0

#[test]
fn test_with_capacity_zero() {
    let set: IndexSet = IndexSet::with_capacity(0);
    assert_eq!(set.map.len(), 0);
}

#[test]
fn test_with_capacity_non_zero() {
    let n = 5;
    let set: IndexSet = IndexSet::with_capacity(n);
    assert_eq!(set.map.capacity(), n);
}

#[test]
fn test_with_capacity_large_number() {
    let n = 1_000_000;
    let set: IndexSet = IndexSet::with_capacity(n);
    assert_eq!(set.map.capacity(), n);
    assert_eq!(set.map.len(), 0);
}

