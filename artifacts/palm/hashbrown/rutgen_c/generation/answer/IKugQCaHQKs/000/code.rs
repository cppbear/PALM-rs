// Answer 0

#[test]
fn test_get_mut_existing_element() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::with_capacity_in(4, DummyAllocator);
    let hash = 42;
    let value = String::from("test");
    table.insert(hash, value.clone(), |v| hash);

    let mut eq = |v: &String| v == &value;
    let result = table.get_mut(hash, eq);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), &mut value);
}

#[test]
fn test_get_mut_non_existing_element() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::with_capacity_in(4, DummyAllocator);
    let hash = 42;

    let mut eq = |_: &String| false;
    let result = table.get_mut(hash, eq);
    assert!(result.is_none());
}

#[test]
fn test_get_mut_multiple_elements() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::with_capacity_in(4, DummyAllocator);
    let hash1 = 42;
    let value1 = String::from("test1");
    table.insert(hash1, value1.clone(), |v| hash1);

    let hash2 = 43;
    let value2 = String::from("test2");
    table.insert(hash2, value2.clone(), |v| hash2);

    let mut eq1 = |v: &String| v == &value1;
    let result1 = table.get_mut(hash1, eq1);
    assert!(result1.is_some());
    assert_eq!(result1.unwrap(), &mut value1);

    let mut eq2 = |v: &String| v == &value2;
    let result2 = table.get_mut(hash2, eq2);
    assert!(result2.is_some());
    assert_eq!(result2.unwrap(), &mut value2);
}

