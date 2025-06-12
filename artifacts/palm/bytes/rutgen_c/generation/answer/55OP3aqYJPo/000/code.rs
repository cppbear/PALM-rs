// Answer 0

#[test]
fn test_promotable_even_to_vec() {
    use core::alloc::Layout;

    #[repr(C)]
    struct TestAlloc {
        data: [u8; 4],
    }

    let layout = Layout::array::<TestAlloc>(1).unwrap();
    let ptr = unsafe { alloc::alloc(layout) as *const u8 };
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        // Initialize the allocated memory
        let test_alloc = &mut *(ptr as *mut TestAlloc);
        test_alloc.data = [1, 2, 3, 4];

        // Call the function under test
        let vec = promotable_even_to_vec(&atomic_ptr, ptr, 4);

        // Check the result
        assert_eq!(vec, [1, 2, 3, 4]);

        // Cleanup
        dealloc(ptr as *mut u8, layout);
    }
}

#[test]
#[should_panic]
fn test_promotable_even_to_vec_invalid_len() {
    use core::alloc::Layout;

    #[repr(C)]
    struct TestAlloc {
        data: [u8; 4],
    }

    let layout = Layout::array::<TestAlloc>(1).unwrap();
    let ptr = unsafe { alloc::alloc(layout) as *const u8 };
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        // Initialize the allocated memory
        let test_alloc = &mut *(ptr as *mut TestAlloc);
        test_alloc.data = [1, 2, 3, 4];

        // This will panic as we are trying to read more than what we have initialized
        let _vec = promotable_even_to_vec(&atomic_ptr, ptr, 5);

        // Cleanup
        dealloc(ptr as *mut u8, layout);
    }
}

