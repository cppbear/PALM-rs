// Answer 0

#[cfg(test)]
fn test_resize_inner_success() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
            Ok(NonNull::new_unchecked(alloc::alloc::alloc(layout)))
        }

        fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            alloc::alloc::dealloc(ptr.as_ptr(), layout)
        }
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout { size: std::mem::size_of::<u64>(), ctrl_align: std::mem::align_of::<u64>() };
    let mut table = unsafe {
        RawTableInner::new_uninitialized(&alloc, table_layout, 4, Fallibility::Infallible).unwrap()
    };

    let hasher = |_: &mut RawTableInner, _: usize| 0u64;

    let result = unsafe {
        table.resize_inner(&alloc, 8, &hasher, Fallibility::Infallible, table_layout)
    };

    assert!(result.is_ok());
}

#[cfg(test)]
fn test_resize_inner_capacity_zero() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
            Ok(NonNull::new_unchecked(alloc::alloc::alloc(layout)))
        }

        fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            alloc::alloc::dealloc(ptr.as_ptr(), layout)
        }
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout { size: std::mem::size_of::<u64>(), ctrl_align: std::mem::align_of::<u64>() };
    let mut table = unsafe {
        RawTableInner::new_uninitialized(&alloc, table_layout, 4, Fallibility::Infallible).unwrap()
    };

    let hasher = |_: &mut RawTableInner, _: usize| 0u64;

    let result = unsafe {
        table.resize_inner(&alloc, 0, &hasher, Fallibility::Infallible, table_layout)
    };

    assert!(result.is_err());
}

#[cfg(test)]
fn test_resize_inner_capacity_overflow() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
            Ok(NonNull::new_unchecked(alloc::alloc::alloc(layout)))
        }

        fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            alloc::alloc::dealloc(ptr.as_ptr(), layout)
        }
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout { size: std::mem::size_of::<u64>(), ctrl_align: std::mem::align_of::<u64>() };
    let mut table = unsafe {
        RawTableInner::new_uninitialized(&alloc, table_layout, 8, Fallibility::Infallible).unwrap()
    };

    let hasher = |_: &mut RawTableInner, _: usize| 0u64;

    let result = unsafe {
        table.resize_inner(&alloc, usize::MAX, &hasher, Fallibility::Infallible, table_layout)
    };

    assert!(result.is_err());
}

