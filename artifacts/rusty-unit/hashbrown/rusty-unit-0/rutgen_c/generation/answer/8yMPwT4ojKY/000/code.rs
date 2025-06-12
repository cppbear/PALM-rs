// Answer 0

#[test]
fn test_into_mut() {
    use crate::hash_map::HashMap;
    use crate::raw::{Allocator, Global, RawTable};
    use std::ptr::NonNull;
  
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: core::alloc::Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: core::alloc::Layout) {}
    }

    let mut raw_table: RawTable<(&str, u32), TestAllocator> = RawTable {
        table: Default::default(),
        alloc: TestAllocator,
        marker: std::marker::PhantomData,
    };
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 100);
    map.insert("b", 200);

    let value: &mut u32;

    match map.raw_entry_mut().from_key(&"a") {
        crate::raw::RawEntryMut::Vacant(_) => panic!(),
        crate::raw::RawEntryMut::Occupied(mut entry) => {
            value = entry.into_mut();
        }
    }
    
    *value += 900;

    assert_eq!(map[&"a"], 1000);
}

#[test]
fn test_into_mut_non_existent_key() {
    use crate::hash_map::HashMap;
    use crate::raw::{Allocator, Global, RawTable};
    use std::ptr::NonNull;
  
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: core::alloc::Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: core::alloc::Layout) {}
    }

    let mut raw_table: RawTable<(&str, u32), TestAllocator> = RawTable {
        table: Default::default(),
        alloc: TestAllocator,
        marker: std::marker::PhantomData,
    };

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 100);
    map.insert("b", 200);

    let result = map.raw_entry_mut().from_key(&"c");
    assert!(matches!(result, crate::raw::RawEntryMut::Vacant(_)));
}

