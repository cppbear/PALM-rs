// Answer 0

#[test]
fn test_free_boxed_slice_valid() {
    use alloc::alloc::{alloc, dealloc, Layout};
    
    let layout = Layout::from_size_align(10, 1).unwrap();
    let buf = unsafe { alloc(layout) };
    
    assert!(!buf.is_null());

    let offset = buf.add(2);
    let len = 5;

    unsafe {
        free_boxed_slice(buf, offset, len);
    }
}

#[test]
#[should_panic]
fn test_free_boxed_slice_invalid_offset() {
    use alloc::alloc::{alloc, dealloc, Layout};

    let layout = Layout::from_size_align(10, 1).unwrap();
    let buf = unsafe { alloc(layout) };

    assert!(!buf.is_null());

    let offset = buf.add(20); // Invalid offset, beyond allocation size
    let len = 5;

    unsafe {
        free_boxed_slice(buf, offset, len);
    }
}

#[test]
fn test_free_boxed_slice_zero_length() {
    use alloc::alloc::{alloc, dealloc, Layout};

    let layout = Layout::from_size_align(10, 1).unwrap();
    let buf = unsafe { alloc(layout) };

    assert!(!buf.is_null());

    let offset = buf.add(2);
    let len = 0;

    unsafe {
        free_boxed_slice(buf, offset, len);
    }
}

