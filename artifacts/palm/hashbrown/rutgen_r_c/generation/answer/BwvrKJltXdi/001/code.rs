// Answer 0

#[test]
fn test_replace_entry_with_updates_value() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("key", 10);

    let entry = match map.entry("key") {
        Entry::Occupied(e) => {
            e.replace_entry_with(|k, v| {
                assert_eq!(k, &"key");
                assert_eq!(v, 10);
                Some(v + 5) // Update the value
            })
        }
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    };

    match entry {
        Entry::Occupied(e) => {
            assert_eq!(e.key(), &"key");
            assert_eq!(e.get(), &15); // The updated value
        }
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    }

    assert_eq!(map["key"], 15);
}

#[test]
fn test_replace_entry_with_removes_entry() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("key", 20);

    let entry = match map.entry("key") {
        Entry::Occupied(e) => {
            e.replace_entry_with(|_k, _v| {
                None // Remove the entry
            })
        }
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    };

    match entry {
        Entry::Vacant(e) => {
            assert_eq!(e.key(), &"key"); // Check the key of the removed entry
        }
        Entry::Occupied(_) => panic!("Expected vacant entry"),
    }

    assert!(!map.contains_key("key")); // The entry should be removed
}

