// Answer 0

#[test]
fn test_with_capacity_in_min_capacity() {
    let capacity = 1;
    let alloc = Global;
    let _result = RawTable::<u8, Global>::with_capacity_in(capacity, alloc);
}

#[test]
fn test_with_capacity_in_small_capacity() {
    let capacity = 16;
    let alloc = Global;
    let _result = RawTable::<u8, Global>::with_capacity_in(capacity, alloc);
}

#[test]
fn test_with_capacity_in_large_capacity() {
    let capacity = 1024;
    let alloc = Global;
    let _result = RawTable::<u8, Global>::with_capacity_in(capacity, alloc);
}

#[test]
fn test_with_capacity_in_power_of_two_capacity() {
    let capacity = 2048; // 2^11
    let alloc = Global;
    let _result = RawTable::<u8, Global>::with_capacity_in(capacity, alloc);
}

#[test]
fn test_with_capacity_in_max_capacity() {
    let capacity = 1 << 30; // 2^30
    let alloc = Global;
    let _result = RawTable::<u8, Global>::with_capacity_in(capacity, alloc);
}

