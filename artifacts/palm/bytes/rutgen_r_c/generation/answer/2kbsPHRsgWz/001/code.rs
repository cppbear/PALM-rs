// Answer 0

#[test]
fn test_promotable_is_unique_when_ref_cnt_is_one() {
    use core::ptr::null_mut;
    use alloc::alloc::{alloc, dealloc, Layout};
    
    // Create a layout for a Shared instance
    let layout = Layout::new::<Shared>();
    
    // Allocate memory for a Shared instance
    unsafe {
        let ptr = alloc(layout) as *mut Shared;
        if ptr.is_null() {
            panic!("Memory allocation failed");
        }

        // Initialize the Shared instance
        (*ptr).buf = null_mut();
        (*ptr).cap = 0;
        (*ptr).ref_cnt = AtomicUsize::new(1); // Set ref_cnt to 1

        // Create an AtomicPtr pointing to the Shared instance
        let atomic_ptr = AtomicPtr::new(ptr as *mut ());

        // Call the function under test
        let result = promotable_is_unique(&atomic_ptr);

        // Assert the expected result
        assert!(result);

        // Clean up
        dealloc(ptr as *mut u8, layout);
    }
}

#[test]
fn test_promotable_is_unique_when_ref_cnt_is_greater_than_one() {
    use core::ptr::null_mut;
    use alloc::alloc::{alloc, dealloc, Layout};
    
    let layout = Layout::new::<Shared>();

    unsafe {
        let ptr = alloc(layout) as *mut Shared;
        if ptr.is_null() {
            panic!("Memory allocation failed");
        }

        (*ptr).buf = null_mut();
        (*ptr).cap = 0;
        (*ptr).ref_cnt = AtomicUsize::new(2); // Set ref_cnt to 2

        let atomic_ptr = AtomicPtr::new(ptr as *mut ());

        let result = promotable_is_unique(&atomic_ptr);

        assert!(!result); // should return false

        dealloc(ptr as *mut u8, layout);
    }
}

#[test]
fn test_promotable_is_unique_with_a_non_arc_kind() {
    use core::ptr::null_mut;
    use alloc::alloc::{alloc, dealloc, Layout};
    
    let layout = Layout::new::<Shared>();

    unsafe {
        let ptr = alloc(layout) as *mut Shared;
        if ptr.is_null() {
            panic!("Memory allocation failed");
        }

        (*ptr).buf = null_mut();
        (*ptr).cap = 0;

        // Pack it as a kind that is not KIND_ARC
        let atomic_ptr = AtomicPtr::new(ptr as *mut () | KIND_VEC);

        let result = promotable_is_unique(&atomic_ptr);

        assert!(result); // should return true, because kind is KIND_VEC

        dealloc(ptr as *mut u8, layout);
    }
}

