// Answer 0

#[test]
fn test_shallow_clone_vec_success() {
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    struct Bytes {
        ptr: *const u8,
        len: usize,
        data: AtomicPtr<Shared>,
        vtable: &'static VTable,
    }

    #[repr(C)]
    struct VTable;

    static SHARED_VTABLE: VTable = VTable;

    let atom = AtomicPtr::new(null_mut());
    let buffer = vec![1u8, 2, 3, 4, 5];
    let len = buffer.len();
    let ptr = buffer.as_ptr(); // This will be the original pointer
    let offset: *const u8 = ptr; // Since this is a test, the offset can just be the same

    unsafe {
        let result = shallow_clone_vec(
            &atom,
            ptr as _,
            buffer.as_ptr() as *mut u8,
            offset,
            len,
        );

        assert_eq!(result.len, len);
        assert_eq!(result.ptr, offset);
    }
}

#[test]
#[should_panic]
fn test_shallow_clone_vec_ptr_misaligned() {
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    struct Bytes {
        ptr: *const u8,
        len: usize,
        data: AtomicPtr<Shared>,
        vtable: &'static VTable,
    }

    #[repr(C)]
    struct VTable;

    static SHARED_VTABLE: VTable = VTable;

    let atom = AtomicPtr::new(null_mut());
    let buffer = vec![1u8, 2, 3, 4, 5];
    let len = buffer.len();
    let ptr = buffer.as_ptr(); // This will be the original pointer
    let offset: *const u8 = ptr as *const u8; // Align this pointer

    // This will lead to misalignment
    let misaligned_ptr = (offset as usize + 1) as *const u8;

    unsafe {
        shallow_clone_vec(
            &atom,
            misaligned_ptr as _,
            buffer.as_ptr() as *mut u8,
            offset,
            len,
        );
    }
}

#[test]
fn test_shallow_clone_vec_compare_exchange_failed() {
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    struct Bytes {
        ptr: *const u8,
        len: usize,
        data: AtomicPtr<Shared>,
        vtable: &'static VTable,
    }

    #[repr(C)]
    struct VTable;

    static SHARED_VTABLE: VTable = VTable;

    // Setup atomic pointer with an initial pointer
    let initial_ptr = Box::into_raw(Box::new(0 as *mut u8));
    let atom: AtomicPtr<()> = AtomicPtr::new(initial_ptr);

    // Create original buffer
    let original_buffer = vec![1u8, 2, 3, 4, 5];
    let len = original_buffer.len();
    let ptr = original_buffer.as_ptr();
    let offset: *const u8 = ptr;

    // Try cloning, it will end up attempting the compare_exchange which should fail
    unsafe {
        // The atom already points to initial_ptr, hence, we expect the compare_exchange to fail
        let result = shallow_clone_vec(
            &atom,
            ptr as _,
            original_buffer.as_ptr() as *mut u8,
            offset,
            len,
        );

        assert_eq!(result.len, len);
        assert_eq!(result.ptr, offset);
    }

    // Clean up
    unsafe {
        Box::from_raw(initial_ptr);
    }
}

