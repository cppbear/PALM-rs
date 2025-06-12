// Answer 0

#[test]
fn test_capacity_with_large_initialization() {
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(1000);
    assert!(set.capacity() >= 1000);
}

#[test]
fn test_capacity_with_small_initialization() {
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(1);
    assert!(set.capacity() >= 1);
}

#[test]
fn test_capacity_with_exact_capacity() {
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(16);
    assert!(set.capacity() >= 16);
}

#[test]
fn test_capacity_with_zero_initialization() {
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(0);
    assert!(set.capacity() >= 0);
}

#[test]
fn test_capacity_should_not_panic() {
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(usize::MAX);
    assert!(set.capacity() > 0); // Ensuring it does not panic and has some capacity
}

