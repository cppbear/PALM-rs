// Answer 0

#[test]
fn test_drop_inner_table_no_items() {
    use std::alloc::Global;

    let allocator = Global;
    let table_layout = TableLayout { size: 0, ctrl_align: 0 };
    
    let mut table = RawTableInner {
        ctrl: NonNull::new_unchecked(Box::into_raw(Box::new(0u8)) as *mut u8),
        bucket_mask: 0,
        growth_left: 0,
        items: 0,
    };

    unsafe {
        table.drop_inner_table::<u8>(&allocator, table_layout);
    }
}

#[test]
#[should_panic]
fn test_drop_inner_table_with_items_panics_on_drop() {
    use std::alloc::Global;

    struct PanickingDrop;

    impl Drop for PanickingDrop {
        fn drop(&mut self) {
            panic!("Drop called");
        }
    }

    let allocator = Global;
    let table_layout = TableLayout { size: 0, ctrl_align: 0 };

    let mut table = RawTableInner {
        ctrl: NonNull::new_unchecked(Box::into_raw(Box::new(PanickingDrop)) as *mut u8),
        bucket_mask: 1,
        growth_left: 0,
        items: 1,
    };

    unsafe {
        table.drop_inner_table::<PanickingDrop>(&allocator, table_layout);
    }
}

#[test]
fn test_drop_inner_table_correct_deallocation() {
    use std::alloc::Global;

    let allocator = Global;
    let table_layout = TableLayout { size: 0, ctrl_align: 0 };

    let mut table = RawTableInner {
        ctrl: NonNull::new_unchecked(Box::into_raw(Box::new(0u8)) as *mut u8),
        bucket_mask: 1,
        growth_left: 0,
        items: 1,
    };

    unsafe {
        table.drop_inner_table::<u8>(&allocator, table_layout);
    }

    // Ensure that after dropping, the allocation can be reused without issues.
    let new_value = Box::new(1u8);
    let ptr = Box::into_raw(new_value);
    unsafe { assert!(!ptr.is_null()) };
}

