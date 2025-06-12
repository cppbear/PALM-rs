// Answer 0

#[test]
fn test_new_uninitialized_success() {
    use std::alloc::{self, Allocator, Global};
    
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let ptr = alloc::alloc(layout);
            if ptr.is_null() {
                Err(())
            } else {
                Ok(NonNull::new_unchecked(ptr))
            }
        }

        // Implement deallocate, etc., as necessary
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout::new::<u8>();
    let buckets = 8; // a power of two
    let fallibility = Fallibility::Infallible;

    let result = unsafe { RawTableInner::new_uninitialized(&alloc, table_layout, buckets, fallibility) };

    match result {
        Ok(table_inner) => {
            assert_eq!(table_inner.bucket_mask, buckets - 1);
            assert_eq!(table_inner.items, 0);
            assert_eq!(table_inner.growth_left, bucket_mask_to_capacity(buckets - 1));
        },
        Err(_) => panic!("Expected Ok result, but got an Err"),
    }
}

#[test]
#[should_panic]
fn test_new_uninitialized_capacity_overflow() {
    use std::alloc::{self, Allocator, Global};
    
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let ptr = alloc::alloc(layout);
            if ptr.is_null() {
                Err(())
            } else {
                Ok(NonNull::new_unchecked(ptr))
            }
        }

        // Implement deallocate, etc., as necessary
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout::new::<u8>();
    let buckets = usize::MAX; // this will cause a capacity overflow
    let fallibility = Fallibility::Infallible;

    unsafe { RawTableInner::new_uninitialized(&alloc, table_layout, buckets, fallibility) };
}

#[test]
#[should_panic]
fn test_new_uninitialized_null_pointer() {
    use std::alloc::{self, Allocator};

    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(()) // Simulate allocation failure
        }

        // Implement deallocate, etc., as necessary
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout::new::<u8>();
    let buckets = 8; // a power of two
    let fallibility = Fallibility::Infallible;

    unsafe { RawTableInner::new_uninitialized(&alloc, table_layout, buckets, fallibility) };
}

