// Answer 0

#[test]
fn test_resize_inner_success() {
    use crate::alloc::{Allocator, Global};
    use std::alloc::{Layout, alloc};

    // Create a mock allocator
    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Implement necessary methods for the Allocator trait
    }

    (MockAllocator);
    let alloc = MockAllocator;

    // Initialize a TableLayout
    let layout = TableLayout { size: 32, ctrl_align: 8 };
    
    // Set up RawTableInner with valid parameters
    let mut raw_table_inner = unsafe {
        RawTableInner::new_uninitialized(&alloc, layout, 8, Fallibility::Infallible).unwrap()
    };

    // Define a hash function for testing
    let hasher = |_: &mut RawTableInner, index: usize| -> u64 { index as u64 };

    // Try resizing to a larger capacity
    let result = unsafe { raw_table_inner.resize_inner(&alloc, 16, &hasher, Fallibility::Infallible, layout) };
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_resize_inner_panic_empty_table() {
    use crate::alloc::{Allocator, Global};
    use std::alloc::{Layout, alloc};

    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Implement necessary methods for the Allocator trait
    }

    let alloc = MockAllocator;

    // Initialize a TableLayout
    let layout = TableLayout { size: 32, ctrl_align: 8 };

    // Create an empty RawTableInner
    let mut raw_table_inner = unsafe {
        RawTableInner::new_uninitialized(&alloc, layout, 0, Fallibility::Infallible).unwrap()
    };

    // Define a hash function for testing
    let hasher = |_: &mut RawTableInner, _: usize| -> u64 { 0 };

    // Attempt to resize with empty table
    unsafe {
        raw_table_inner.resize_inner(&alloc, 0, &hasher, Fallibility::Infallible, layout).unwrap();
    }
}

#[test]
#[should_panic]
fn test_resize_inner_exceeding_capacity() {
    use crate::alloc::{Allocator, Global};
    use std::alloc::{Layout, alloc};

    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Implement necessary methods for the Allocator trait
    }

    let alloc = MockAllocator;

    // Initialize a TableLayout
    let layout = TableLayout { size: 32, ctrl_align: 8 };

    // Set up RawTableInner
    let mut raw_table_inner = unsafe {
        RawTableInner::new_uninitialized(&alloc, layout, 4, Fallibility::Infallible).unwrap()
    };

    // Define a hash function for testing
    let hasher = |_: &mut RawTableInner, _: usize| -> u64 { 0 };

    // Attempt to resize to smaller capacity than items
    let result = unsafe {
        raw_table_inner.resize_inner(&alloc, 2, &hasher, Fallibility::Infallible, layout)
    };
    assert!(result.is_err());
}

