// Answer 0

#[test]
fn test_promotable_odd_drop_arc() {
    use core::ptr::null_mut;
    use alloc::boxed::Box;
    
    struct AtomicPtrWrapper {
        ptr: AtomicPtr<()>,
    }
    
    impl AtomicPtrWrapper {
        fn new(ptr: *const () , kind: usize) -> Self {
            let shared = kind as *const () as *mut ();
            AtomicPtrWrapper {
                ptr: AtomicPtr::new(shared),
            }
        }
        
        fn with_mut<F>(&mut self, f: F) 
        where
            F: FnOnce(&mut *mut ()),
        {
            let data = self.ptr.get_mut();
            f(data);
        }
    }
    
    unsafe {
        let ptr = null_mut();
        let mut data = AtomicPtrWrapper::new(ptr as *const (), KIND_ARC);
        promotable_odd_drop(&mut data.ptr, ptr, 0);
    }
}

#[test]
fn test_promotable_odd_drop_vec() {
    use core::ptr::null_mut;
    use alloc::vec::Vec;
    
    struct AtomicPtrWrapper {
        ptr: AtomicPtr<()>,
    }

    impl AtomicPtrWrapper {
        fn new(ptr: *const (), kind: usize) -> Self {
            let shared = kind as *const () as *mut ();
            AtomicPtrWrapper {
                ptr: AtomicPtr::new(shared),
            }
        }

        fn with_mut<F>(&mut self, f: F)
        where
            F: FnOnce(&mut *mut ()),
        {
            let data = self.ptr.get_mut();
            f(data);
        }
    }
    
    unsafe {
        let vec: Vec<u8> = Vec::with_capacity(10);
        let ptr = vec.as_ptr() as *const ();
        let mut data = AtomicPtrWrapper::new(ptr, KIND_VEC);
        promotable_odd_drop(&mut data.ptr, ptr, 10);
    }
}

