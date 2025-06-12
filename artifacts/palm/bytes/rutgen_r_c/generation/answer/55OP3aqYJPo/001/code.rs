// Answer 0

#[test]
fn test_promotable_even_to_vec_valid_input() {
    use core::sync::atomic::{AtomicPtr, Ordering};
    
    let data = AtomicPtr::new(Box::into_raw(Box::new(1)));
    let input_ptr = Box::into_raw(Box::new(42u8));
    let len = 1;

    unsafe {
        let result = promotable_even_to_vec(&data, input_ptr, len);
        assert_eq!(result.len(), len);
        assert_eq!(result[0], 42);
        // Clean up
        dealloc(input_ptr as *mut u8, Layout::new::<u8>());
        dealloc(data.load(Ordering::Acquire), Layout::new::<i32>());
    }
}

#[should_panic]
#[test]
fn test_promotable_even_to_vec_invalid_ptr() {
    use core::sync::atomic::{AtomicPtr, Ordering};

    let data = AtomicPtr::new(Box::into_raw(Box::new(1)));
    let invalid_ptr = ptr::null(); // Invalid pointer
    let len = 1;

    unsafe {
        let _ = promotable_even_to_vec(&data, invalid_ptr, len);
    }
}

#[test]
fn test_promotable_even_to_vec_zero_length() {
    use core::sync::atomic::{AtomicPtr, Ordering};

    let data = AtomicPtr::new(Box::into_raw(Box::new(1)));
    let input_ptr = Box::into_raw(Box::new(42u8));
    let len = 0;

    unsafe {
        let result = promotable_even_to_vec(&data, input_ptr, len);
        assert!(result.is_empty());
        // Clean up
        dealloc(input_ptr as *mut u8, Layout::new::<u8>());
        dealloc(data.load(Ordering::Acquire), Layout::new::<i32>());
    }
}

#[test]
fn test_promotable_even_to_vec_large_input() {
    use core::sync::atomic::{AtomicPtr, Ordering};

    let data = AtomicPtr::new(Box::into_raw(Box::new(1)));
    let len = 1024;
    let input_vec: Vec<u8> = (0..len as u8).collect();
    let input_ptr = input_vec.as_ptr() as *const u8;

    unsafe {
        let result = promotable_even_to_vec(&data, input_ptr, len);
        assert_eq!(result.len(), len);
        assert_eq!(result, input_vec);
        // Clean up
        dealloc(data.load(Ordering::Acquire), Layout::new::<i32>());
    }
}

