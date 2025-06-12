// Answer 0

#[test]
fn test_buckets_with_minimum_bucket_mask() {
    let table = RawTable::<u32>::new_in(Global);
    let expected_buckets = 1; // bucket_mask = 0
    let actual_buckets = table.buckets();
}

#[test]
fn test_buckets_with_small_bucket_mask() {
    let mut table = RawTable::<u32>::new_in(Global);
    let small_mask = 3; // bucket_mask = 3
    table.table.bucket_mask = small_mask;
    let expected_buckets = 4; // bucket_mask = 3
    let actual_buckets = table.buckets();
}

#[test]
fn test_buckets_with_large_bucket_mask() {
    let mut table = RawTable::<u32>::new_in(Global);
    let large_mask = usize::MAX - 1; // bucket_mask = 2^n - 2, where n is the bit size of usize
    table.table.bucket_mask = large_mask;
    let expected_buckets = usize::MAX; // bucket_mask = 2^n - 2
    let actual_buckets = table.buckets();
}

#[test]
fn test_buckets_with_maximum_bucket_mask() {
    let mut table = RawTable::<u32>::new_in(Global);
    let max_mask = usize::MAX; // bucket_mask = 2^n - 1
    table.table.bucket_mask = max_mask;
    let expected_buckets = usize::MAX + 1; // bucket_mask = 2^n - 1
    let actual_buckets = table.buckets();
}

