// Answer 0

#[test]
fn test_entry_debug_vacant() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut hashmap: HashMap<&str, &str, DefaultHashBuilder, TestAllocator> = HashMap::new();
    let vacant_entry = VacantEntry {
        hash: 0,
        key: "a",
        table: &mut hashmap,
    };
    let entry = Entry::Vacant(vacant_entry);
    
    let mut result = String::new();
    let f = &mut fmt::Formatter::new(&mut result);
    
    entry.fmt(f).unwrap();
    
    assert!(result.contains("Vacant"));
}

#[test]
fn test_entry_debug_occupied() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut hashmap: HashMap<&str, &str, DefaultHashBuilder, TestAllocator> = HashMap::new();
    hashmap.insert("a", "first");

    let occupied_entry = OccupiedEntry {
        hash: 0,
        elem: Bucket { /* initialization */ },
        table: &mut hashmap,
    };
    let entry = Entry::Occupied(occupied_entry);

    let mut result = String::new();
    let f = &mut fmt::Formatter::new(&mut result);
    
    entry.fmt(f).unwrap();

    assert!(result.contains("Occupied"));
}

