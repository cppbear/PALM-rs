// Answer 0

#[test]
fn test_allocator_with_default_allocator() {
    let alloc = Global;
    let table = RawTable::<i32>::new_in(alloc);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_with_capacity() {
    let alloc = Global;
    let table = RawTable::with_capacity_in(1024, alloc);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_with_min_capacity() {
    let alloc = Global;
    let table = RawTable::with_capacity_in(1, alloc);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_with_max_capacity() {
    let alloc = Global;
    let table = RawTable::with_capacity_in(1 << 30, alloc);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_with_zero_capacity() {
    let alloc = Global;
    let table = RawTable::<u8>::with_capacity_in(0, alloc);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_non_null() {
    let alloc = Global;
    let table = RawTable::<f64>::with_capacity_in(10, alloc);
    let allocator = table.allocator();
}

