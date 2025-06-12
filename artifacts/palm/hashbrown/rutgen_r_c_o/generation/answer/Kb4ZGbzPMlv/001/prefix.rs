// Answer 0

#[test]
fn test_data_end_with_min_buckets() {
    let alloc = Global; 
    let mut table: RawTable<u8, _> = RawTable::with_capacity_in(1, alloc);
    let end = table.data_end();
}

#[test]
fn test_data_end_with_small_buckets() {
    let alloc = Global; 
    let mut table: RawTable<u8, _> = RawTable::with_capacity_in(4, alloc);
    let end = table.data_end();
}

#[test]
fn test_data_end_with_medium_buckets() {
    let alloc = Global; 
    let mut table: RawTable<u8, _> = RawTable::with_capacity_in(1024, alloc);
    let end = table.data_end();
}

#[test]
fn test_data_end_with_large_buckets() {
    let alloc = Global; 
    let mut table: RawTable<u8, _> = RawTable::with_capacity_in(1 << 20, alloc);
    let end = table.data_end();
}

#[test]
fn test_data_end_with_max_buckets() {
    let alloc = Global; 
    let mut table: RawTable<u8, _> = RawTable::with_capacity_in(1 << 30, alloc);
    let end = table.data_end();
}

#[test]
fn test_data_end_with_items() {
    let alloc = Global; 
    let mut table: RawTable<u8, _> = RawTable::with_capacity_in(16, alloc);
    for i in 0..8 {
        let _ = table.insert(i as u64, i as u8, |&x| x);
    }
    let end = table.data_end();
}

#[test]
fn test_data_end_exceed_buckets() {
    let alloc = Global; 
    let mut table: RawTable<u8, _> = RawTable::with_capacity_in(8, alloc);
    for i in 0..6 {
        let _ = table.insert(i as u64, i as u8, |&x| x);
    }
    let end = table.data_end();
}

