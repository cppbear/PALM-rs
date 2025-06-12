// Answer 0

#[test]
fn test_insert_key() {
    use crate::hash_map::{HashMap, RawOccupiedEntryMut};
    use std::rc::Rc;
    use std::ptr;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let key_one = Rc::new("a");
    let key_two = Rc::new("a");

    let mut map: HashMap<Rc<&str>, u32, DummyAllocator> = HashMap::new();
    map.insert(key_one.clone(), 10);

    assert_eq!(map[&key_one], 10);
    assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_key = o.insert_key(key_two.clone());
            assert!(Rc::ptr_eq(&old_key, &key_one));
        }
    }

    assert_eq!(map[&key_two], 10);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);
}

#[test]
#[should_panic]
fn test_insert_key_panic_on_vacant_entry() {
    use crate::hash_map::{HashMap, RawOccupiedEntryMut};
    use std::rc::Rc;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let key = Rc::new("a");

    let mut map: HashMap<Rc<&str>, u32, DummyAllocator> = HashMap::new();

    match map.raw_entry_mut().from_key(&key) {
        RawEntryMut::Vacant(_) => {
            let _ = o.insert_key(key.clone()); // This will panic as the entry is vacant.
        }
        RawEntryMut::Occupied(_) => {}
    }
}

