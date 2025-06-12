// Answer 0

#[test]
fn test_with_capacity_in_zero_capacity() {
    let bump = bumpalo::Bump::new();
    let table = HashTable::with_capacity_in(0, &bump);
}

#[test]
fn test_with_capacity_in_small_capacity() {
    let bump = bumpalo::Bump::new();
    let table = HashTable::with_capacity_in(1, &bump);
}

#[test]
fn test_with_capacity_in_medium_capacity() {
    let bump = bumpalo::Bump::new();
    let table = HashTable::with_capacity_in(10, &bump);
}

#[test]
fn test_with_capacity_in_large_capacity() {
    let bump = bumpalo::Bump::new();
    let table = HashTable::with_capacity_in(100, &bump);
}

#[test]
fn test_with_capacity_in_maximum_capacity() {
    let bump = bumpalo::Bump::new();
    let max_capacity = std::usize::MAX;
    let table = HashTable::with_capacity_in(max_capacity, &bump);
}

