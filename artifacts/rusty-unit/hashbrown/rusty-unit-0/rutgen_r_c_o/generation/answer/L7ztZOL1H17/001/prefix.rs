// Answer 0

#[test]
fn test_next_with_non_empty_iter() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0u8)) as *mut u8))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
            Box::from_raw(ptr.as_ptr());
        }
    }

    let mut table = RawTable::new_in(TestAllocator);
    let mut iter = RawIter {
        iter: RawIterRange { /* initialize as per context */ },
        items: 1,
    };
    
    let mut raw_extract = RawExtractIf { iter, table: &mut table };

    let mut f = |item: &mut u8| {
        *item += 1; // satisfying f(item.as_mut()) == true
        true
    };
    
    let result = raw_extract.next(&mut f);
}

#[test]
fn test_next_with_multiple_items() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0u8)) as *mut u8))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
            Box::from_raw(ptr.as_ptr());
        }
    }

    let mut table = RawTable::new_in(TestAllocator);
    let mut iter = RawIter {
        iter: RawIterRange { /* initialize with multiple items */ },
        items: 2,
    };
    
    let mut raw_extract = RawExtractIf { iter, table: &mut table };

    let mut f = |item: &mut u8| {
        *item += 2; // satisfying f(item.as_mut()) == true
        true
    };
    
    let result = raw_extract.next(&mut f);
}

#[test]
fn test_next_with_specific_condition() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(5u8)) as *mut u8))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
            Box::from_raw(ptr.as_ptr());
        }
    }

    let mut table = RawTable::new_in(TestAllocator);
    let mut iter = RawIter {
        iter: RawIterRange { /* initialize with condition 5 */ },
        items: 1,
    };
    
    let mut raw_extract = RawExtractIf { iter, table: &mut table };

    let mut f = |item: &mut u8| {
        *item == 5 // satisfying f(item.as_mut()) == true
    };
    
    let result = raw_extract.next(&mut f);
}

#[test]
fn test_next_with_empty_iter() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(3u8)) as *mut u8))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
            Box::from_raw(ptr.as_ptr());
        }
    }

    let mut table = RawTable::new_in(TestAllocator);
    let mut iter = RawIter {
        iter: RawIterRange { /* initialize as empty */ },
        items: 0,
    };
    
    let mut raw_extract = RawExtractIf { iter, table: &mut table };

    let mut f = |item: &mut u8| {
        *item > 0 // cannot satisfy as iter is empty
    };
    
    let result = raw_extract.next(&mut f);
}

