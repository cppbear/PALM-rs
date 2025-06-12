// Answer 0

#[test]
fn test_clone_from_impl_empty_table() {
    unsafe {
        let source: RawTable<u8> = RawTable::new_in(Global);
        let mut dest: RawTable<u8> = RawTable::new_in(Global);
        dest.clone_from_impl(&source);
    }
}

#[test]
fn test_clone_from_impl_zero_buckets() {
    unsafe {
        let source: RawTable<u8> = RawTable::with_capacity_in(0, Global);
        let mut dest: RawTable<u8> = RawTable::with_capacity_in(0, Global);
        dest.clone_from_impl(&source);
    }
}

#[test]
fn test_clone_from_impl_no_elements() {
    unsafe {
        let source: RawTable<u8> = RawTable::with_capacity_in(4, Global);
        let mut dest: RawTable<u8> = RawTable::with_capacity_in(4, Global);
        dest.clone_from_impl(&source);
    }
}

