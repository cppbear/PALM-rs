// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new();
    
    let entry = map.entry("test_key");
    
    assert_eq!(entry.or_insert_with(|| 42), &mut 42);
    assert_eq!(map["test_key"], 42);
}

#[test]
fn test_or_insert_with_existing_entry() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new();
    map.insert("test_key", 21);
    
    let entry = map.entry("test_key");
    
    assert_eq!(entry.or_insert_with(|| 100), &mut 21);
    assert_eq!(map["test_key"], 21);
}

#[test]
fn test_or_insert_with_empty_string_key() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new();

    let entry = map.entry("");
    
    assert_eq!(entry.or_insert_with(|| -1), &mut -1);
    assert_eq!(map[""], -1);
}

