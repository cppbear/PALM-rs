// Answer 0

#[test]
fn test_release_shared_with_ref_cnt_greater_than_one() {
    struct TestShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let buf = Box::into_raw(Box::new([0u8; 10])); // Create a buffer
    let shared = Box::new(TestShared {
        buf,
        cap: 10,
        ref_cnt: AtomicUsize::new(2), // Set reference count greater than 1
    });

    let ptr = Box::into_raw(shared); // Get raw pointer

    unsafe {
        release_shared(ptr); // Call the function under test
    }
}

#[test]
fn test_release_shared_with_multiple_references() {
    struct TestShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let buf1 = Box::into_raw(Box::new([1u8; 10])); // Create a buffer
    let buf2 = Box::into_raw(Box::new([2u8; 10])); // Create another buffer
    let shared1 = Box::new(TestShared {
        buf: buf1,
        cap: 10,
        ref_cnt: AtomicUsize::new(3), // Set reference count greater than 1
    });

    let shared2 = Box::new(TestShared {
        buf: buf2,
        cap: 10,
        ref_cnt: AtomicUsize::new(3), // Set another reference count
    });

    let ptr1 = Box::into_raw(shared1); // Get raw pointer
    let ptr2 = Box::into_raw(shared2); // Get another raw pointer

    unsafe {
        release_shared(ptr1); // Call the function for the first shared object
        release_shared(ptr2); // Call the function for the second shared object
    }
}

#[test]
fn test_release_shared_with_initial_ref_cnt_of_two() {
    struct TestShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let buf = Box::into_raw(Box::new([0u8; 10])); // Create a buffer
    let shared = Box::new(TestShared {
        buf,
        cap: 10,
        ref_cnt: AtomicUsize::new(2), // Set reference count to 2
    });

    let ptr = Box::into_raw(shared); // Get raw pointer

    unsafe {
        release_shared(ptr); // Call the function under test
    }
}

