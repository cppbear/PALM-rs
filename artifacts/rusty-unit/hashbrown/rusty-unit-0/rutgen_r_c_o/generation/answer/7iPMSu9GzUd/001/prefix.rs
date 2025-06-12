// Answer 0

#[test]
fn test_with_capacity_zero() {
    let set: HashSet<i32> = HashSet::with_capacity(0);
}

#[test]
fn test_with_capacity_small() {
    let set: HashSet<i32> = HashSet::with_capacity(5);
}

#[test]
fn test_with_capacity_medium() {
    let set: HashSet<i32> = HashSet::with_capacity(50);
}

#[test]
fn test_with_capacity_large() {
    let set: HashSet<i32> = HashSet::with_capacity(500);
}

#[test]
fn test_with_capacity_xlarge() {
    let set: HashSet<i32> = HashSet::with_capacity(5000);
}

#[test]
fn test_with_capacity_maximum() {
    let set: HashSet<i32> = HashSet::with_capacity(50000);
}

