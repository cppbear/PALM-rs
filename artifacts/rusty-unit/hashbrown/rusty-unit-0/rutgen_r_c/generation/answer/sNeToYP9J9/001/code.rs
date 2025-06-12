// Answer 0

#[test]
fn test_occupied_entry_get() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0u8)) as *mut u8))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
            drop(Box::from_raw(ptr.as_ptr()));
        }
    }

    struct MockHashMap {
        data: HashMap<&'static str, u32, DefaultHashBuilder, MockAllocator>,
    }

    impl MockHashMap {
        fn new() -> Self {
            MockHashMap {
                data: HashMap::new(),
            }
        }

        fn insert(&mut self, key: &'static str, value: u32) {
            self.data.insert(key, value);
        }

        fn entry(&mut self, key: &'static str) -> map::OccupiedEntry<&'static str, u32, DefaultHashBuilder, MockAllocator> {
            self.data.entry(key).or_insert(0)
        }
    }

    let mut mock_map = MockHashMap::new();
    mock_map.insert("poneyland", 1);

    match mock_map.entry("poneyland") {
        map::Entry::Vacant(_) => panic!(),
        map::Entry::Occupied(entry) => assert_eq!(entry.get(), &"poneyland"),
    }

    mock_map.insert("another", 2);

    match mock_map.entry("another") {
        map::Entry::Vacant(_) => panic!(),
        map::Entry::Occupied(entry) => assert_eq!(entry.get(), &"another"),
    }
}

#[test]
#[should_panic]
fn test_occupied_entry_get_panics_on_non_existent() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0u8)) as *mut u8))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
            drop(Box::from_raw(ptr.as_ptr()));
        }
    }

    struct MockHashMap {
        data: HashMap<&'static str, u32, DefaultHashBuilder, MockAllocator>,
    }

    impl MockHashMap {
        fn new() -> Self {
            MockHashMap {
                data: HashMap::new(),
            }
        }

        fn entry(&mut self, key: &'static str) -> map::OccupiedEntry<&'static str, u32, DefaultHashBuilder, MockAllocator> {
            self.data.entry(key).or_insert(0)
        }
    }

    let mut mock_map = MockHashMap::new();

    // Attempt to get a reference from a non-existent entry
    match mock_map.entry("non_existent_key") {
        map::Entry::Occupied(entry) => {
            // This should not occur, as the entry does not exist
            let _ = entry.get(); 
        },
        map::Entry::Vacant(_) => {
            panic!("Entry should be occupied");
        }
    }
}

