// Answer 0

#[test]
fn test_capacity_zero() {
    let set: HashSet<i32> = HashSet::with_capacity(0);
    let _ = set.capacity();
}

#[test]
fn test_capacity_small() {
    let set: HashSet<i32> = HashSet::with_capacity(100);
    let _ = set.capacity();
}

#[test]
fn test_capacity_medium() {
    let set: HashSet<i32> = HashSet::with_capacity(1_000);
    let _ = set.capacity();
}

#[test]
fn test_capacity_large() {
    let set: HashSet<i32> = HashSet::with_capacity(10_000);
    let _ = set.capacity();
}

#[test]
fn test_capacity_max() {
    let set: HashSet<i32> = HashSet::with_capacity(usize::MAX);
    let _ = set.capacity();
}

