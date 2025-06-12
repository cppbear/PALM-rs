// Answer 0

#[test]
fn test_free_boxed_slice_valid() {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr;

    let layout = Layout::from_size_align(10, 1).unwrap();
    let buf = unsafe { alloc(layout) };
    
    let offset = unsafe { buf.add(2) }; // Pointing 2 bytes into the allocated buffer
    let len = 5;

    unsafe {
        free_boxed_slice(buf, offset, len);
    }
}

#[should_panic]
#[test]
fn test_free_boxed_slice_invalid_offset() {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr;

    let layout = Layout::from_size_align(10, 1).unwrap();
    let buf = unsafe { alloc(layout) };

    let offset = unsafe { buf.add(12) }; // Pointing out of bounds of allocated buffer
    let len = 5;

    unsafe {
        free_boxed_slice(buf, offset, len);
    }
}

