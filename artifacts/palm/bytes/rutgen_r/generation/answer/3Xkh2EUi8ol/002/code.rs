// Answer 0

#[test]
fn test_owned_clone_boundary_condition() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::mem::MaybeUninit;

    struct OwnedLifetime {
        ref_cnt: AtomicUsize,
    }

    struct Bytes {
        ptr: *const u8,
        len: usize,
        data: AtomicPtr<()>,
        vtable: &'static VTable,
    }

    struct VTable;

    static OWNED_VTABLE: VTable = VTable;

    // Create an owned instance with ref_cnt at usize::MAX >> 1
    let ref_count_initial = usize::MAX >> 1;
    let owned_instance = OwnedLifetime {
        ref_cnt: AtomicUsize::new(ref_count_initial),
    };
    
    let atomic_ptr = AtomicPtr::new(&owned_instance as *const _ as *mut _);

    // Prepare the test data
    let test_ptr: *const u8 = MaybeUninit::zeroed().as_ptr();
    let test_len = 10;

    // Call the function under test
    let result = unsafe { owned_clone(&atomic_ptr, test_ptr, test_len) };

    // Validate the results
    assert_eq!(result.ptr, test_ptr);
    assert_eq!(result.len, test_len);
    assert_eq!(result.data.load(Ordering::Relaxed), atomic_ptr.load(Ordering::Relaxed));
    assert_eq!(owned_instance.ref_cnt.load(Ordering::Relaxed), ref_count_initial + 1);
}

