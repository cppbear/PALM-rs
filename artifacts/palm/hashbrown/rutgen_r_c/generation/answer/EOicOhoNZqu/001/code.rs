// Answer 0

#[test]
fn test_capacity_with_items_and_growth_left_non_zero() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(&mut 0 as *mut u8))
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let alloc = TestAllocator;
    let mut raw_table = RawTable::<u8, TestAllocator>::new_in(alloc);
    
    raw_table.table.items = 5;
    raw_table.table.growth_left = 10;
    
    assert_eq!(raw_table.capacity(), 15);
}

#[test]
fn test_capacity_with_items_zero_and_growth_left_non_zero() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(&mut 0 as *mut u8))
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let alloc = TestAllocator;
    let mut raw_table = RawTable::<u8, TestAllocator>::new_in(alloc);
    
    raw_table.table.items = 0;
    raw_table.table.growth_left = 10;
    
    assert_eq!(raw_table.capacity(), 10);
}

#[test]
fn test_capacity_with_items_non_zero_and_growth_left_zero() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(&mut 0 as *mut u8))
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let alloc = TestAllocator;
    let mut raw_table = RawTable::<u8, TestAllocator>::new_in(alloc);
    
    raw_table.table.items = 5;
    raw_table.table.growth_left = 0;
    
    assert_eq!(raw_table.capacity(), 5);
}

#[test]
fn test_capacity_with_items_zero_and_growth_left_zero() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(&mut 0 as *mut u8))
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let alloc = TestAllocator;
    let mut raw_table = RawTable::<u8, TestAllocator>::new_in(alloc);
    
    raw_table.table.items = 0;
    raw_table.table.growth_left = 0;
    
    assert_eq!(raw_table.capacity(), 0);
}

