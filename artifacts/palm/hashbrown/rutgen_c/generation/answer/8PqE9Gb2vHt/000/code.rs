// Answer 0

#[test]
fn test_fmt_occupied_entry() {
    use std::ptr::NonNull;

    struct DummyAllocator;
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let key = 42;
    let value = "TestValue";
    let bucket = Bucket {
        ptr: NonNull::from(&mut (key, value)),
    };
    
    let mut map = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    let occupied_entry = OccupiedEntry {
        hash: 0,
        elem: bucket,
        table: &mut map,
    };

    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new();
        occupied_entry.fmt(&mut formatter).unwrap();
        output = formatter.finish().unwrap();
    }

    assert!(output.contains("OccupiedEntry"));
    assert!(output.contains("key: 42"));
    assert!(output.contains("value: TestValue"));
}

