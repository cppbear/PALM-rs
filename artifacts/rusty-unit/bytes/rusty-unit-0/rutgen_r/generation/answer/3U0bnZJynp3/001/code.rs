// Answer 0

#[test]
fn test_shared_to_vec_valid_input() {
    use std::ptr::null;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct TestData {
        buf: [u8; 4],
    }

    let data = AtomicPtr::new(Box::into_raw(Box::new(TestData { buf: [1, 2, 3, 4] })));
    let ptr = data.load(Ordering::Relaxed).cast::<TestData>();
    let len = 4; // Valid length

    let result = unsafe { shared_to_vec(&data, (*ptr).buf.as_ptr(), len) };
    assert_eq!(result, vec![1, 2, 3, 4]);

    // Clean up
    unsafe { Box::from_raw(ptr) };
}

#[test]
#[should_panic]
fn test_shared_to_vec_null_pointer() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    let data = AtomicPtr::new(std::ptr::null_mut());
    let ptr = std::ptr::null();
    let len = 0; // Invalid length

    unsafe { shared_to_vec(&data, ptr, len) };
}

#[test]
fn test_shared_to_vec_zero_length() {
    use std::ptr::null;
    use std::sync::atomic::{AtomicPtr, Ordering};

    let data = AtomicPtr::new(Box::into_raw(Box::new(())));
    let ptr = null();
    let len = 0; // Valid zero length

    let result = unsafe { shared_to_vec(&data, ptr, len) };
    assert_eq!(result, vec![]);
    
    // Clean up
    unsafe { Box::from_raw(data.load(Ordering::Relaxed)) };
}

#[test]
#[should_panic]
fn test_shared_to_vec_exceeding_length() {
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, Ordering};

    let data = AtomicPtr::new(Box::into_raw(Box::new([0u8; 3])));
    let ptr = data.load(Ordering::Relaxed).cast::<[u8; 3]>();
    let len = 4; // Exceeding length

    let _result = unsafe { shared_to_vec(&data, (*ptr).as_ptr(), len) };

    // Clean up
    unsafe { Box::from_raw(ptr) };
}

