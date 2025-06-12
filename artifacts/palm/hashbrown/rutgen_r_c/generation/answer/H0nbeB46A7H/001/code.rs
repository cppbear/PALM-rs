// Answer 0

#[test]
fn test_insert_with_existing_key() {
    use crate::hash_map::HashMap;
    use crate::raw::{RawOccupiedEntryMut, Global};
    use core::ptr::NonNull;
    use std::alloc::Layout;
    use std::marker::PhantomData;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32> = [("a", 100)].into();

    let entry = map.raw_entry_mut().from_key(&"a");
    match entry {
        RawEntryMut::Vacant(_) => panic!("Expected an occupied entry"),
        RawEntryMut::Occupied(mut occupied) => {
            let old_value = occupied.insert(200);
            assert_eq!(old_value, 100);
            assert_eq!(map[&"a"], 200);
        }
    }
}

#[test]
fn test_insert_replaces_value() {
    use crate::hash_map::HashMap;
    use crate::raw::{RawOccupiedEntryMut, Global};
    use core::ptr::NonNull;
    use std::alloc::Layout;
    use std::marker::PhantomData;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32> = [("b", 300)].into();

    let entry = map.raw_entry_mut().from_key(&"b");
    match entry {
        RawEntryMut::Vacant(_) => panic!("Expected an occupied entry"),
        RawEntryMut::Occupied(mut occupied) => {
            let old_value = occupied.insert(400);
            assert_eq!(old_value, 300);
            assert_eq!(map[&"b"], 400);
        }
    }
}

#[test]
fn test_insert_with_identical_value() {
    use crate::hash_map::HashMap;
    use crate::raw::{RawOccupiedEntryMut, Global};
    use core::ptr::NonNull;
    use std::alloc::Layout;
    use std::marker::PhantomData;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32> = [("c", 150)].into();

    let entry = map.raw_entry_mut().from_key(&"c");
    match entry {
        RawEntryMut::Vacant(_) => panic!("Expected an occupied entry"),
        RawEntryMut::Occupied(mut occupied) => {
            let old_value = occupied.insert(150);
            assert_eq!(old_value, 150);
            assert_eq!(map[&"c"], 150);
        }
    }
}

