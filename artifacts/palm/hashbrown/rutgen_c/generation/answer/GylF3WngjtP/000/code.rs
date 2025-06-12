// Answer 0

#[test]
fn test_get_many_unchecked_mut() {
    use std::alloc::{Global, Layout};
    use std::ptr::NonNull;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                Err(())
            } else {
                Ok(NonNull::new(ptr).unwrap())
            }
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(TestAllocator);

    // Setup test data
    let hashes = [1, 2, 3, 4, 5];
    for &hash in &hashes {
        unsafe {
            table.insert(hash, hash as i32, |x| *x);
        }
    }

    let mut eq_called = [false; 5];
    let eq = |index: usize, value: &i32| {
        eq_called[index] = true;
        *value == (index + 1) as i32
    };

    // Perform the operation
    unsafe {
        let results = table.get_many_unchecked_mut::<5>(hashes, eq);
        
        // Validate the results
        for (i, result) in results.iter().enumerate() {
            assert!(eq_called[i], "eq function was not called for index {}", i);
            match result {
                Some(&mut val) => assert_eq!(val, (i + 1) as i32),
                None => panic!("Expected value for hash {}, but got None", hashes[i]),
            }
        }
    }
}

