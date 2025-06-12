// Answer 0

#[test]
fn test_occupied_entry_debug_fmt() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, MockAllocator> = HashMap::default();
    map.insert(1, 10);

    let occupied_entry = OccupiedEntry {
        hash: 0,
        elem: Bucket { /* ... initialization ... */ },
        table: &mut map,
    };

    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        occupied_entry.fmt(&mut formatter).unwrap();
    }
    
    assert!(output.contains("OccupiedEntry"));
    assert!(output.contains("value"));
}

