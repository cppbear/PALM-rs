// Answer 0

#[test]
fn test_remove_with_valid_bucket() {
    let alloc = Global;
    let mut table: RawTable<i32, _> = RawTable::new_in(alloc);
    unsafe {
        let bucket = table.insert(0, 42, |x| *x as u64);
        let (value, insert_slot) = table.remove(bucket);
        let expected_index = table.bucket_index(&bucket);
    }
}

#[test]
fn test_remove_with_multiple_items() {
    let alloc = Global;
    let mut table: RawTable<i32, _> = RawTable::new_in(alloc);
    unsafe {
        let bucket1 = table.insert(0, 10, |x| *x as u64);
        let bucket2 = table.insert(1, 20, |x| *x as u64);
        let (value, insert_slot) = table.remove(bucket1);
        let expected_index = table.bucket_index(&bucket1);
    }
}

#[test]
fn test_remove_when_table_is_full() {
    let alloc = Global;
    let mut table: RawTable<i32, _> = RawTable::with_capacity_in(4, alloc);
    unsafe {
        let bucket1 = table.insert(0, 1, |x| *x as u64);
        let bucket2 = table.insert(1, 2, |x| *x as u64);
        let bucket3 = table.insert(2, 3, |x| *x as u64);
        let bucket4 = table.insert(3, 4, |x| *x as u64);
        let (value, insert_slot) = table.remove(bucket4);
        let expected_index = table.bucket_index(&bucket4);
    }
}

#[test]
#[should_panic]
fn test_remove_non_existent_bucket() {
    let alloc = Global;
    let mut table: RawTable<i32, _> = RawTable::new_in(alloc);
    unsafe {
        let invalid_bucket = Bucket { ptr: NonNull::dangling() };
        let _ = table.remove(invalid_bucket);
    }
}

#[test]
fn test_remove_zero_sized_type() {
    struct ZeroSized;

    let alloc = Global;
    let mut table: RawTable<ZeroSized, _> = RawTable::new_in(alloc);
    unsafe {
        let bucket = table.insert(0, ZeroSized, |_: &ZeroSized| 0);
        let (value, insert_slot) = table.remove(bucket);
        let expected_index = table.bucket_index(&bucket);
    }
}

#[test]
fn test_remove_under_high_load() {
    let alloc = Global;
    let mut table: RawTable<u64, _> = RawTable::with_capacity_in(16, alloc);
    unsafe {
        for i in 0..16 {
            table.insert(i, i * 10, |x| *x);
        }
        let bucket_to_remove = table.bucket(0);
        let (value, insert_slot) = table.remove(bucket_to_remove);
    }
}

