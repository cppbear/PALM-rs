// Answer 0

#[test]
fn test_into_allocation_non_empty_singleton() {
    let raw_table = RawTable::<u8>::with_capacity_in(16, Global);
    let result = raw_table.clone().into_allocation();
}

#[test]
fn test_into_allocation_large_capacity() {
    let raw_table = RawTable::<u8>::with_capacity_in(1000, Global);
    let result = raw_table.clone().into_allocation();
}

#[test]
fn test_into_allocation_near_max_capacity() {
    let raw_table = RawTable::<u8>::with_capacity_in(usize::MAX - 1, Global);
    let result = raw_table.clone().into_allocation();
}

#[test]
fn test_into_allocation_non_power_of_two_buckets() {
    let raw_table = RawTable::<u8>::with_capacity_in(3, Global);
    let result = raw_table.clone().into_allocation();
}

#[test]
#[should_panic]
fn test_into_allocation_zero_buckets() {
    let raw_table = RawTable::<u8>::new_in(Global);
    let result = raw_table.into_allocation();
}

#[test]
fn test_into_allocation_power_of_two_buckets() {
    let raw_table = RawTable::<u8>::with_capacity_in(8, Global);
    let result = raw_table.clone().into_allocation();
}

