// Answer 0

#[test]
fn test_with_capacity_small() {
    let _table = RawTable::<u32>::with_capacity(1);
}

#[test]
fn test_with_capacity_mid_range() {
    let _table = RawTable::<u32>::with_capacity(1024);
}

#[test]
fn test_with_capacity_larger() {
    let _table = RawTable::<u32>::with_capacity(1 << 20); // 1 MiB
}

#[test]
fn test_with_capacity_max() {
    let _table = RawTable::<u32>::with_capacity(1 << 30); // 1 GiB
}

