// Answer 0

#[test]
fn test_insert_key_existing_value() {
    use std::rc::Rc;
    
    struct DummyAllocator;
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            // Implement allocation logic if needed, returning an appropriate pointer.
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
            // Implement deallocation logic if needed.
        }
    }
    
    let key_one = Rc::new("key_one");
    let key_two = Rc::new("key_two");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);
    
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_key = o.insert_key(key_two.clone());
            assert!(Rc::ptr_eq(&old_key, &key_one));
        }
    }
    assert_eq!(map[&key_two], 10);
}

#[test]
fn test_insert_key_empty_map() {
    use std::rc::Rc;

    struct DummyAllocator;
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            // Implement allocation logic if needed, returning an appropriate pointer.
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
            // Implement deallocation logic if needed.
        }
    }

    let key = Rc::new("key");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

    match map.raw_entry_mut().from_key(&key) {
        RawEntryMut::Vacant(mut e) => {
            assert_eq!(e.insert_key(key.clone()), key);
            assert_eq!(map[&key], 0);
        }
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_insert_key_same_key() {
    use std::rc::Rc;

    struct DummyAllocator;
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            // Implement allocation logic if needed, returning an appropriate pointer.
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
            // Implement deallocation logic if needed.
        }
    }

    let key = Rc::new("duplicate_key");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key.clone(), 20);

    match map.raw_entry_mut().from_key(&key) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_key = o.insert_key(key.clone());
            assert!(Rc::ptr_eq(&old_key, &key));
            assert_eq!(map[&key], 20);
        }
    }
}

