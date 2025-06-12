// Answer 0

#[test]
fn test_owned_clone_with_high_ref_cnt() {
    let mut owned_lifetime = OwnedLifetime {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1),
        drop: unsafe { std::mem::transmute::<fn(*mut ()), fn(*mut ())>(abort) },
    };

    let data = AtomicPtr::new(&mut owned_lifetime as *mut _ as *mut ());
    let ptr: *const u8 = core::ptr::null(); // Using null pointer as a safe dummy input
    let len: usize = 0; // Using 0 length as a safe dummy value

    unsafe {
        owned_clone(&data, ptr, len);
    }
}

#[test]
fn test_owned_clone_with_one_below_max() {
    let mut owned_lifetime = OwnedLifetime {
        ref_cnt: AtomicUsize::new((usize::MAX >> 1) - 1),
        drop: unsafe { std::mem::transmute::<fn(*mut ()), fn(*mut ())>(abort) },
    };

    let data = AtomicPtr::new(&mut owned_lifetime as *mut _ as *mut ());
    let ptr: *const u8 = core::ptr::null();
    let len: usize = 0; // Using 0 length as a safe dummy value

    unsafe {
        owned_clone(&data, ptr, len);
    }
}

#[should_panic]
#[test]
fn test_owned_clone_with_max_ref_cnt() {
    let mut owned_lifetime = OwnedLifetime {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1),
        drop: unsafe { std::mem::transmute::<fn(*mut ()), fn(*mut ())>(abort) },
    };

    let data = AtomicPtr::new(&mut owned_lifetime as *mut _ as *mut ());
    let ptr: *const u8 = core::ptr::null();
    let len: usize = 0; // Using 0 length as a safe dummy value

    unsafe {
        owned_clone(&data, ptr, len); // This should panic due to incrementing old_cnt above the limit
    }
}

