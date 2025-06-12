// Answer 0

#[test]
fn test_resize_inner_success_with_infallible() {
    let allocator = Global;
    let layout = TableLayout { size: 100, ctrl_align: 8 };
    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 4);
    raw_table.items = 2; // Assuming some items are already present

    unsafe {
        let result = raw_table.resize_inner(
            &allocator,
            8,
            &|_, idx| idx as u64, // Simple hash function
            Fallibility::Infallible,
            layout,
        );
    }
}

#[test]
fn test_resize_inner_success_with_fallible() {
    let allocator = Global;
    let layout = TableLayout { size: 50, ctrl_align: 8 };
    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 4);
    raw_table.items = 3; // Assuming there are some items

    unsafe {
        let result = raw_table.resize_inner(
            &allocator,
            10,
            &|_, idx| idx as u64,
            Fallibility::Fallible,
            layout,
        );
    }
}

#[test]
#[should_panic]
fn test_resize_inner_panic_due_to_zero_capacity() {
    let allocator = Global;
    let layout = TableLayout { size: 100, ctrl_align: 8 };
    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 4);
    raw_table.items = 2; // Fill some items to cause panic on resize

    unsafe {
        let _ = raw_table.resize_inner(
            &allocator,
            0, // Invalid capacity
            &|_, idx| idx as u64,
            Fallibility::Infallible,
            layout,
        );
    }
}

#[test]
#[should_panic]
fn test_resize_inner_panic_due_to_empty_bucket_with_items() {
    let allocator = Global;
    let layout = TableLayout { size: 50, ctrl_align: 8 };
    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 4);
    raw_table.items = 5; // More items than buckets

    unsafe {
        let _ = raw_table.resize_inner(
            &allocator,
            4, // Invalid capacity
            &|_, idx| idx as u64,
            Fallibility::Infallible,
            layout,
        );
    }
}

