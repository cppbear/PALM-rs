// Answer 0

#[test]
fn test_hashset_new_empty() {
    let set: HashSet<i32> = HashSet::new();
}

#[test]
fn test_hashset_with_capacity_zero() {
    let set: HashSet<i32> = HashSet::with_capacity(0);
}

#[test]
fn test_hashset_with_capacity_small() {
    let set: HashSet<i32> = HashSet::with_capacity(1);
}

#[test]
fn test_hashset_with_capacity_large() {
    let set: HashSet<i32> = HashSet::with_capacity(100);
}

