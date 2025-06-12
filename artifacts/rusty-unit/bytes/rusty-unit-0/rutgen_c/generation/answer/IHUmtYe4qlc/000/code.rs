// Answer 0

#[test]
fn test_shared_is_unique_when_unique() {
    use core::ptr::null_mut;
    
    struct AtomicPtrWrapper {
        ptr: AtomicPtr<()>,
    }

    let mut shared = Shared {
        buf: null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1),
    };
    let shared_ptr = AtomicPtr::new(&mut shared as *mut _ as *mut ());
    let wrapper = AtomicPtrWrapper { ptr: shared_ptr };

    assert_eq!(unsafe { shared_is_unique(&wrapper.ptr) }, true);
}

#[test]
fn test_shared_is_unique_when_not_unique() {
    use core::ptr::null_mut;
    
    struct AtomicPtrWrapper {
        ptr: AtomicPtr<()>,
    }

    let mut shared = Shared {
        buf: null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(2),
    };
    let shared_ptr = AtomicPtr::new(&mut shared as *mut _ as *mut ());
    let wrapper = AtomicPtrWrapper { ptr: shared_ptr };

    assert_eq!(unsafe { shared_is_unique(&wrapper.ptr) }, false);
}

