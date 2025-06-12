// Answer 0

#[test]
fn test_capacity_empty_table() {
    struct CustomAllocator;
    
    impl Allocator for CustomAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(unsafe { ptr::null_mut() }).unwrap())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let allocator = CustomAllocator;
    let table: RawTable<i32, CustomAllocator> = RawTable::new_in(allocator);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_capacity_non_empty_table() {
    struct CustomAllocator;
    
    impl Allocator for CustomAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(unsafe { ptr::null_mut() }).unwrap())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let allocator = CustomAllocator;
    let table: RawTable<i32, CustomAllocator> = RawTable::with_capacity_in(10, allocator);
    assert_eq!(table.capacity(), 10);
} 

#[test]
fn test_capacity_after_insertion() {
    struct CustomAllocator;
    
    impl Allocator for CustomAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(unsafe { ptr::null_mut() }).unwrap())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let allocator = CustomAllocator;
    let mut table: RawTable<i32, CustomAllocator> = RawTable::with_capacity_in(10, allocator);
    
    // Simulating an insertion operation that would increase items
    unsafe {
        table.table.items = 5; // Simulate there are now 5 items
        table.table.growth_left = 5; // And 5 more can be held
    }

    assert_eq!(table.capacity(), 10);
}

#[test]
fn test_capacity_with_growth_left() {
    struct CustomAllocator;
    
    impl Allocator for CustomAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(unsafe { ptr::null_mut() }).unwrap())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let allocator = CustomAllocator;
    let mut table: RawTable<i32, CustomAllocator> = RawTable::with_capacity_in(10, allocator);
    
    // Simulating the growth left directly
    unsafe {
        table.table.items = 3; // Simulate there are now 3 items
        table.table.growth_left = 2; // And 2 more can be held
    }

    assert_eq!(table.capacity(), 5);
}

