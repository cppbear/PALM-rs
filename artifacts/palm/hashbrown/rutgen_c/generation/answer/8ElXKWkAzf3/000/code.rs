// Answer 0

#[test]
fn test_get_key_value() {
    use std::ptr::NonNull;
    use std::alloc::{Allocator, Global, Layout};
    use core::marker::PhantomData;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            unimplemented!()
        }
    }

    struct TestRawTable<K, V, A: Allocator> {
        // Simplified for the context of the test
        data: Vec<(K, V)>,
        alloc: A,
    }

    struct TestBucket<T> {
        ptr: NonNull<T>,
    }

    impl<T> TestBucket<T> {
        fn as_ref(&self) -> &T {
            unsafe { self.ptr.as_ref() }
        }
    }

    struct TestRawOccupiedEntryMut<'a, K, V, S, A: Allocator> {
        elem: TestBucket<(K, V)>,
        table: &'a mut TestRawTable<K, V, A>,
        hash_builder: &'a S,
    }

    impl<'a, K, V, S, A: Allocator> TestRawOccupiedEntryMut<'a, K, V, S, A> {
        fn get_key_value(&self) -> (&K, &V) {
            unsafe {
                let (key, value) = self.elem.as_ref();
                (key, value)
            }
        }
    }

    let key = "a";
    let value = 100;
    let bucket = TestBucket {
        ptr: NonNull::from(&(&key, &value)),
    };

    let mut table = TestRawTable {
        data: vec![(key, value)],
        alloc: TestAllocator,
    };

    let entry = TestRawOccupiedEntryMut {
        elem: bucket,
        table: &mut table,
        hash_builder: &(),
    };

    let (entry_key, entry_value) = entry.get_key_value();
    assert_eq!(entry_key, &key);
    assert_eq!(entry_value, &value);
}

