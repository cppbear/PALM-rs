// Answer 0

#[test]
fn test_allocation_size_empty_table() {
    let allocator = Global; // Using the global allocator
    let table: HashTable<i32> = HashTable::new_in(allocator);
    let size = table.allocation_size();
}

#[test]
fn test_allocation_size_small_capacity() {
    let allocator = Global;
    let table: HashTable<i32> = HashTable::with_capacity_in(1, allocator);
    let size = table.allocation_size();
}

#[test]
fn test_allocation_size_large_capacity() {
    let allocator = Global;
    let capacity = 1 << 30; // 2^30
    let table: HashTable<i32> = HashTable::with_capacity_in(capacity, allocator);
    let size = table.allocation_size();
}

#[test]
fn test_allocation_size_some_capacity() {
    let allocator = Global;
    let table: HashTable<i32> = HashTable::with_capacity_in(100, allocator);
    let size = table.allocation_size();
}

#[test]
fn test_allocation_size_after_insertion() {
    let allocator = Global;
    let mut table: HashTable<i32> = HashTable::new_in(allocator);
    table.insert_unique(1, 10, |v| *v as u64);
    let size = table.allocation_size();
}

#[test]
fn test_allocation_size_full_table() {
    let allocator = Global;
    let table: HashTable<i32> = HashTable::with_capacity_in(1 << 29, allocator);
    for i in 0..(1 << 29) {
        table.insert_unique(i as u64, i, |v| *v as u64);
    }
    let size = table.allocation_size();
}

