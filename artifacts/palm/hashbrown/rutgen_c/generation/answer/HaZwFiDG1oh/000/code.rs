// Answer 0

#[test]
fn test_occupied_entry_debug_fmt() {
    use core::{alloc::Layout, ptr::NonNull};

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    struct DummyBucket<T> {
        value: T,
    }

    impl<T> Bucket<T> {
        fn as_ref(&self) -> &T {
            unsafe { &*self.ptr.as_ptr() }
        }
    }

    let allocator = DummyAllocator;
    let value = 42;
    let bucket = Bucket {
        ptr: NonNull::new(Box::into_raw(Box::new(value))).unwrap(),
    };

    let mut hash_table = HashTable {
        raw: RawTable::new(), // Assuming RawTable has a new method
    };

    let occupied_entry = OccupiedEntry {
        hash: 0,
        bucket,
        table: &mut hash_table,
    };

    let debug_str = format!("{:?}", occupied_entry);
    assert!(debug_str.contains("OccupiedEntry"));
    assert!(debug_str.contains("value: 42"));

    // Clean up the allocated memory
    unsafe {
        allocator.deallocate(bucket.ptr, Layout::new::<i32>());
    }
}

