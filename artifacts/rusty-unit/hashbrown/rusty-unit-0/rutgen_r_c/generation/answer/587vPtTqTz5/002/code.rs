// Answer 0

#[test]
fn test_get_inner_non_empty_table() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    struct TestKey(u32);
    struct TestValue(String);

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &TestKey) -> bool {
            self.0 == other.0
        }
    }

    let mut map: HashMap<TestKey, TestValue, DefaultHasher, TestAllocator> = HashMap {
        hash_builder: DefaultHasher::new(),
        table: RawTable {
            table: RawTableInner::default(), // Assume there's a default implementation
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    // Inserting a value to ensure the table is not empty
    map.insert(TestKey(1), TestValue("Test".to_string()));

    // Performing the get_inner operation
    let result = map.get_inner(&TestKey(1));

    assert!(result.is_some());
    assert_eq!(result.unwrap().1.0, "Test");
}

#[test]
fn test_get_inner_non_existent_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    struct TestKey(u32);
    struct TestValue(String);

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &TestKey) -> bool {
            self.0 == other.0
        }
    }

    let mut map: HashMap<TestKey, TestValue, DefaultHasher, TestAllocator> = HashMap {
        hash_builder: DefaultHasher::new(),
        table: RawTable {
            table: RawTableInner::default(), // Assume there's a default implementation
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    // Inserting a value to ensure the table is not empty
    map.insert(TestKey(1), TestValue("Test".to_string()));

    // Performing the get_inner operation with a non-existent key
    let result = map.get_inner(&TestKey(2));

    assert!(result.is_none());
}

