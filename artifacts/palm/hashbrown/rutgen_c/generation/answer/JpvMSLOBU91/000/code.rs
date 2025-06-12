// Answer 0

#[test]
fn test_fmt_vacant_entry() {
    use crate::hash_map::{Entry, HashMap};

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new();
    let entry = Entry::Vacant(VacantEntry {
        hash: 0,
        key: "a",
        table: &mut map,
    });

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", entry);
    assert!(result.is_ok());
    assert!(output.contains("Entry"));
    assert!(output.contains("Vacant"));
}

#[test]
fn test_fmt_occupied_entry() {
    use crate::hash_map::{Entry, HashMap};

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new();
    map.insert("a", 100);
    let entry = Entry::Occupied(OccupiedEntry {
        hash: 0,
        elem: Bucket::new(("a", 100)),
        table: &mut map,
    });

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", entry);
    assert!(result.is_ok());
    assert!(output.contains("Entry"));
    assert!(output.contains("Occupied"));
}

