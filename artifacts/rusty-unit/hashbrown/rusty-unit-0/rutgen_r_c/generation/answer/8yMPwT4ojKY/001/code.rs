// Answer 0

#[test]
fn test_into_mut() {
    use std::ptr::NonNull;
    use std::alloc::{self, Layout};
    use core::marker::PhantomData;

    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let ptr = unsafe { alloc::alloc(layout) };
            if ptr.is_null() {
                Err(())
            } else {
                Ok(NonNull::new(ptr).unwrap())
            }
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let mut raw_table: RawTable<(&str, u32), MockAllocator> = RawTable { 
        table: RawTableInner::new(), 
        alloc: MockAllocator, 
        marker: PhantomData 
    };

    let key = "a";
    let value = 100;
    let bucket = Bucket { ptr: NonNull::new(Box::into_raw(Box::new((key, value)))).unwrap() };
    
    let mut occupied_entry = RawOccupiedEntryMut {
        elem: bucket,
        table: &mut raw_table,
        hash_builder: &(),
    };

    let mut_val: &mut u32 = occupied_entry.into_mut();
    
    assert_eq!(*mut_val, 100);
    *mut_val += 900;

    assert_eq!(*mut_val, 1000);
}

#[test]
#[should_panic]
fn test_into_mut_invalid_access() {
    use std::ptr::NonNull;
    use std::alloc::{self, Layout};
    use core::marker::PhantomData;

    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let ptr = unsafe { alloc::alloc(layout) };
            if ptr.is_null() {
                Err(())
            } else {
                Ok(NonNull::new(ptr).unwrap())
            }
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let mut raw_table: RawTable<(&str, u32), MockAllocator> = RawTable { 
        table: RawTableInner::new(), 
        alloc: MockAllocator, 
        marker: PhantomData 
    };

    let bucket = Bucket { ptr: NonNull::dangling() }; // Invalid pointer
    let occupied_entry = RawOccupiedEntryMut {
        elem: bucket,
        table: &mut raw_table,
        hash_builder: &(),
    };

    let _mut_val: &mut u32 = occupied_entry.into_mut(); // This should panic
}

