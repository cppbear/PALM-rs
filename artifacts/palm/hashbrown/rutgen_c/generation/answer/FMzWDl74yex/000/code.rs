// Answer 0

#[test]
fn test_into_key() {
    use std::ptr::NonNull;
    use std::alloc::{Allocator, Global, Layout};
    use std::marker::PhantomData;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            unimplemented!()
        }
    }

    struct DummyTable<T, A: Allocator> {
        _marker: PhantomData<(T, A)>,
    }

    struct RawOccupiedEntryMutTest<'a, K, V, S, A: Allocator> {
        elem: Bucket<(K, V)>,
        table: &'a mut DummyTable<(K, V), A>,
        hash_builder: &'a S,
    }

    impl<'a, K, V, S, A: Allocator> RawOccupiedEntryMutTest<'a, K, V, S, A> {
        pub fn new(
            key: K,
            value: V,
            table: &'a mut DummyTable<(K, V), A>,
            hash_builder: &'a S,
        ) -> Self {
            // Simulating the creation of a bucket
            let bucket = Bucket {
                ptr: NonNull::new(Box::into_raw(Box::new((key, value))) as *mut (K, V)).unwrap(),
            };
            Self {
                elem: bucket,
                table,
                hash_builder,
            }
        }

        pub fn into_key(self) -> &'a mut K {
            unsafe { &mut self.elem.ptr.as_mut().0 }
        }
    }

    let mut dummy_table = DummyTable { _marker: PhantomData };
    let key = String::from("example_key");
    let value = 42;
    let hash_builder = ();

    let entry_mut = RawOccupiedEntryMutTest::new(key.clone(), value, &mut dummy_table, &hash_builder);
    let inside_key: &mut String = entry_mut.into_key();

    assert_eq!(inside_key, &mut key);
}

