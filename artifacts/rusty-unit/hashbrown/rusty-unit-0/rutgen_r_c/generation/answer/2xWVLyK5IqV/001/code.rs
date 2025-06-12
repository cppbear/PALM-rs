// Answer 0

#[test]
fn test_or_insert_with_key_vacant_entry() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<String, usize, DefaultHashBuilder, TestAllocator> = HashMap::new();

    // Key is vacant, should insert using default function
    let reference = map.entry_ref("testkey");
    let result = reference.or_insert_with_key(|key| key.len());
    assert_eq!(*result, 8); // Length of "testkey"

    // Ensure the value is in the map
    assert_eq!(map["testkey"], 8);
}

#[test]
fn test_or_insert_with_key_existing_entry() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<String, usize, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("existing_key".to_owned(), 42);

    // Key exists, should not modify value
    let reference = map.entry_ref("existing_key");
    let result = reference.or_insert_with_key(|key| key.len());
    assert_eq!(*result, 42); // Existing value should be returned

    // Ensure that value remains unchanged
    assert_eq!(map["existing_key"], 42);
}

#[test]
#[should_panic]
fn test_or_insert_with_key_panic_due_to_borrow() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<String, usize, DefaultHashBuilder, TestAllocator> = HashMap::new();

    // Panic test when the reference is invalid or borrowed improperly
    let reference = map.entry_ref("invalid_key");
    let _result = reference.or_insert_with_key(|_key| panic!("This is a panic!"));
}

