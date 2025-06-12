// Answer 0

#[test]
fn test_get_mut() {
    use std::alloc::{Allocator as StdAllocator, Global, Layout};
    use std::ptr::NonNull;

    struct TestAllocator;

    unsafe impl StdAllocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            std::alloc::Global::allocate(layout)
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::Global::deallocate(ptr, layout)
        }
    }

    let mut table = RawTable::<(&str, u32), TestAllocator>::new();
    table.insert(("key1", 10));
    
    let mut entry = RawOccupiedEntryMut {
        elem: Bucket {
            ptr: NonNull::new(&mut table[0]).unwrap(),
        },
        table: &mut table,
        hash_builder: &(),
    };
    
    let value = entry.get_mut();
    *value += 5;

    assert_eq!(*value, 15);
}

#[test]
#[should_panic]
fn test_get_mut_panic() {
    let mut table = RawTable::<(&str, u32), Global>::new();
    
    let mut entry = RawOccupiedEntryMut {
        elem: Bucket {
            ptr: NonNull::dangling(),
        },
        table: &mut table,
        hash_builder: &(),
    };
    
    let _value = entry.get_mut(); // This should panic due to dangling pointer
}

