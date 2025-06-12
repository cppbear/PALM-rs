// Answer 0

#[test]
fn test_is_bucket_full_valid_index() {
    let raw_table: RawTable<u8> = RawTable::new_in(Global);
    let max_buckets = raw_table.buckets(); 
    
    unsafe {
        for index in 0..max_buckets {
            raw_table.is_bucket_full(index);
        }
    }
}

#[test]
#[should_panic]
fn test_is_bucket_full_out_of_bounds() {
    let raw_table: RawTable<u8> = RawTable::new_in(Global);
    let max_buckets = raw_table.buckets(); 
    let out_of_bounds_index = max_buckets;

    unsafe {
        raw_table.is_bucket_full(out_of_bounds_index);
    }
}

#[test]
fn test_is_bucket_full_zero_index() {
    let raw_table: RawTable<u8> = RawTable::new_in(Global);
    unsafe {
        raw_table.is_bucket_full(0);
    }
}

#[test]
fn test_is_bucket_full_boundary_index() {
    let raw_table: RawTable<u8> = RawTable::new_in(Global);
    let max_buckets = raw_table.buckets() - 1; 

    unsafe {
        raw_table.is_bucket_full(max_buckets);
    }
}

