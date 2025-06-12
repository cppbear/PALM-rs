// Answer 0

#[test]
fn test_capacity_with_zero_capacity() {
    let table = HashTable::<i32>::with_capacity_in(0, Global);
    let _ = table.capacity();
}

#[test]
fn test_capacity_with_small_capacity() {
    let table = HashTable::<i32>::with_capacity_in(1, Global);
    let _ = table.capacity();
}

#[test]
fn test_capacity_with_large_capacity() {
    let table = HashTable::<i32>::with_capacity_in(100, Global);
    let _ = table.capacity();
}

#[test]
fn test_capacity_with_max_capacity() {
    let table = HashTable::<i32>::with_capacity_in(usize::MAX, Global);
    let _ = table.capacity();
}

#[test]
fn test_capacity_with_high_capacity() {
    let high_capacity = usize::MAX / 2;
    let table = HashTable::<i32>::with_capacity_in(high_capacity, Global);
    let _ = table.capacity();
}

#[test]
fn test_capacity_with_uninitialized_table() {
    let table: HashTable<i32> = unsafe { HashTable::new_uninitialized(Global, 16, Fallibility::default()).unwrap() };
    let _ = table.capacity();
}

