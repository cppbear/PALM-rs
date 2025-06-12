// Answer 0

#[test]
fn test_with_capacity_in_zero_capacity() {
    let allocator = Global;
    let set: HashSet<i32> = HashSet::with_capacity_in(0, allocator);
}

#[test]
fn test_with_capacity_in_small_capacity() {
    let allocator = Global;
    let set: HashSet<i32> = HashSet::with_capacity_in(1, allocator);
}

#[test]
fn test_with_capacity_in_large_capacity() {
    let allocator = Global;
    let set: HashSet<i32> = HashSet::with_capacity_in(1000, allocator);
}

#[test]
fn test_with_capacity_in_non_zero_capacity() {
    let allocator = Global;
    let set: HashSet<i32> = HashSet::with_capacity_in(500, allocator);
}

#[test]
fn test_with_capacity_in_large_prime_capacity() {
    let allocator = Global;
    let set: HashSet<i32> = HashSet::with_capacity_in(997, allocator);
}

