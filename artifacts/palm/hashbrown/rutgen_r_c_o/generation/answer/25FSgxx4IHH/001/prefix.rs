// Answer 0

#[test]
fn test_erase_no_drop_with_single_bucket() {
    let mut table = RawTable::<u32>::with_capacity_in(1, Global);
    let bucket = unsafe { table.bucket(0) };
    unsafe {
        table.insert(0, 10, |x| *x);
        table.erase_no_drop(&bucket);
    }
}

#[test]
fn test_erase_no_drop_with_multiple_buckets() {
    let mut table = RawTable::<u32>::with_capacity_in(4, Global);
    let bucket = unsafe { table.bucket(2) };
    unsafe {
        table.insert(0, 20, |x| *x);
        table.insert(1, 30, |x| *x);
        table.erase_no_drop(&bucket);
    }
}

#[test]
fn test_erase_no_drop_bucket_full() {
    let mut table = RawTable::<u32>::with_capacity_in(8, Global);
    unsafe {
        table.insert(0, 10, |x| *x);
        table.insert(1, 20, |x| *x);
        table.insert(2, 30, |x| *x);
        table.insert(3, 40, |x| *x);
        let bucket = table.bucket(1);
        table.erase_no_drop(&bucket);
    }
}

#[test]
fn test_erase_no_drop_with_growth_left() {
    let mut table = RawTable::<u32>::with_capacity_in(4, Global);
    let bucket = unsafe { table.bucket(0) };
    unsafe {
        table.insert(0, 10, |x| *x);
        table.insert(1, 20, |x| *x);
        table.erase_no_drop(&bucket);
    }
}

#[test]
fn test_erase_no_drop_empty_bucket() {
    let mut table = RawTable::<u32>::with_capacity_in(2, Global);
    let bucket = unsafe { table.bucket(0) };
    unsafe {
        table.erase_no_drop(&bucket);
    }
}

#[test]
fn test_erase_no_drop_multiple_erases() {
    let mut table = RawTable::<u32>::with_capacity_in(4, Global);
    unsafe {
        table.insert(0, 10, |x| *x);
        table.insert(1, 20, |x| *x);
        let bucket1 = table.bucket(0);
        let bucket2 = table.bucket(1);
        table.erase_no_drop(&bucket1);
        table.erase_no_drop(&bucket2);
    }
}

