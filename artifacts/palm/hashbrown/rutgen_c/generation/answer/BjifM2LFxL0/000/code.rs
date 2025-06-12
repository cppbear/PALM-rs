// Answer 0

#[test]
fn test_fmt_occupied_error() {
    use std::fmt;
    use std::ptr::NonNull;

    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    struct MockHashMap<K, V, S, A: Allocator> {
        _marker: PhantomData<(K, V, S, A)>,
    }

    struct MockEntry<K, V> {
        key: K,
        value: V,
    }

    impl<K: Debug, V: Debug, S, A: Allocator> OccupiedEntry<'_, K, V, S, A> {
        fn new(hash: u64, key: K, value: V, table: &mut MockHashMap<K, V, S, A>) -> Self {
            OccupiedEntry {
                hash,
                elem: Bucket(MockEntry { key, value }),
                table,
            }
        }
    }

    struct Bucket<T>(T);

    let key = String::from("test_key");
    let value = String::from("existing_value");
    let mut table = MockHashMap::<String, String, DefaultHashBuilder, MockAllocator> { _marker: PhantomData };

    let occupied_entry = OccupiedEntry::new(0, key.clone(), value.clone(), &mut table);
    let occupied_error = OccupiedError {
        entry: occupied_entry,
        value: String::from("new_value"),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", occupied_error);

    assert!(result.is_ok());
    assert!(output.contains("failed to insert \"new_value\", key \"test_key\" already exists with value \"existing_value\""));
}

