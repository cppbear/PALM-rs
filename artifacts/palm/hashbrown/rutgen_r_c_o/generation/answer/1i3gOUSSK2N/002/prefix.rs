// Answer 0

#[test]
fn test_raw_entry_mut_occupied_fmt() {
    use hashbrown::{hash_map::RawEntryMut, HashMap, raw::RawTable};
    use core::ptr::NonNull;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            // Dummy implementation
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // Dummy implementation
        }
    }

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    let mut raw_table: RawTable<(&str, i32), DummyAllocator> = RawTable::new();

    let occupied_entry = RawOccupiedEntryMut {
        elem: Bucket::new(("key1", 1)),
        table: &mut raw_table,
        hash_builder: &(),
    };

    let entry = RawEntryMut::Occupied(occupied_entry);
    let mut formatter = fmt::Formatter::new();

    let _ = entry.fmt(&mut formatter);
}

#[test]
fn test_raw_entry_mut_empty_table_occupied_fmt() {
    use hashbrown::{hash_map::RawEntryMut, HashMap, raw::RawTable};
    use core::ptr::NonNull;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut raw_table: RawTable<(&str, i32), DummyAllocator> = RawTable::new();

    let occupied_entry = RawOccupiedEntryMut {
        elem: Bucket::new(("key_empty", 0)),
        table: &mut raw_table,
        hash_builder: &(),
    };

    let entry = RawEntryMut::Occupied(occupied_entry);
    let mut formatter = fmt::Formatter::new();

    let _ = entry.fmt(&mut formatter);
}

