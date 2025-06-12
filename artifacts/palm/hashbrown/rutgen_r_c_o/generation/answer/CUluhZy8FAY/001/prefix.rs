// Answer 0

#[test]
fn test_erase_with_initialized_item() {
    let allocator = Global;
    let mut table = RawTable::with_capacity_in(4, allocator);
    unsafe {
        let item = Bucket {
            ptr: NonNull::new(Box::into_raw(Box::new(42))).unwrap(),
        };
        table.erase(item);
    }
}

#[test]
fn test_erase_with_uninitialized_item() {
    let allocator = Global;
    let mut table = RawTable::with_capacity_in(4, allocator);
    unsafe {
        let item = Bucket {
            ptr: NonNull::new_unchecked(ptr::null_mut()),
        };
        table.erase(item);
    }
}

#[should_panic]
#[test]
fn test_erase_with_item_that_triggers_panic_on_drop() {
    struct PanicOnDrop;
    impl Drop for PanicOnDrop {
        fn drop(&mut self) {
            panic!("panic on drop");
        }
    }

    let allocator = Global;
    let mut table = RawTable::with_capacity_in(4, allocator);
    unsafe {
        let item = Bucket {
            ptr: NonNull::new(Box::into_raw(Box::new(PanicOnDrop))).unwrap(),
        };
        table.erase(item);
    }
}

#[test]
fn test_erase_with_decremental_growth_left() {
    let allocator = Global;
    let mut table = RawTable::with_capacity_in(8, allocator);
    unsafe {
        for i in 0..3 {
            let item = Bucket {
                ptr: NonNull::new(Box::into_raw(Box::new(i))).unwrap(),
            };
            table.erase(item);
        }
    }
}

#[test]
fn test_erase_with_edge_case_buckets() {
    let allocator = Global;
    let mut table1 = RawTable::with_capacity_in(1, allocator);
    let mut table2 = RawTable::with_capacity_in(2, allocator);
    unsafe {
        let item1 = Bucket {
            ptr: NonNull::new(Box::into_raw(Box::new(1))).unwrap(),
        };
        table1.erase(item1);

        let item2 = Bucket {
            ptr: NonNull::new(Box::into_raw(Box::new(2))).unwrap(),
        };
        table2.erase(item2);
    }
}

